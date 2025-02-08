use crate::utils::{
    controller::{Node, NodeDrone, NodeHost, NodeType, SimulationController, Topology},
    DroneOptions,
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use std::collections::HashMap;
use std::panic;
use wg_2024::{
    config::Config, controller::DroneEvent, drone::Drone, network::NodeId, packet::Packet,
};

pub fn init_network<D: Drone>(config: &Config) -> SimulationController {
    let topology = init_topology(config);
    let mut nodes = HashMap::new();
    let mut packets = HashMap::new();
    let (drone_send, drone_recv) = unbounded();

    for drone in config.drone.iter() {
        packets.insert(drone.id, unbounded());
    }
    for client in config.client.iter() {
        packets.insert(client.id, unbounded());
    }
    for server in config.server.iter() {
        packets.insert(server.id, unbounded());
    }

    let options = drone_options(config, &mut nodes, &packets, drone_send.clone());
    let pool = ThreadPoolBuilder::new().build().unwrap();

    pool.spawn(move || {
        // ignore drone panic to not crash the whole tester
        _ = panic::catch_unwind(|| {
            options.into_par_iter().panic_fuse().for_each(|opt| {
                D::new(
                    opt.id,
                    opt.controller_send,
                    opt.controller_recv,
                    opt.packet_recv,
                    opt.packet_send,
                    opt.pdr,
                )
                .run();
            })
        })
    });

    for client in config.client.iter() {
        let neighbor_packet_send = client
            .connected_drone_ids
            .iter()
            .cloned()
            .map(|id| (id, packets[&id].0.clone()))
            .collect();
        let (packet_send, packet_recv) = packets[&client.id].clone();
        nodes.insert(
            client.id,
            Node {
                packet_send,
                node_type: NodeType::Host(NodeHost {
                    packet_send: neighbor_packet_send,
                    packet_recv,
                }),
            },
        );
    }
    for server in config.server.iter() {
        let neighbor_packet_send = server
            .connected_drone_ids
            .iter()
            .cloned()
            .map(|id| (id, packets[&id].0.clone()))
            .collect();
        let (packet_send, packet_recv) = packets[&server.id].clone();
        nodes.insert(
            server.id,
            Node {
                packet_send,
                node_type: NodeType::Host(NodeHost {
                    packet_send: neighbor_packet_send,
                    packet_recv,
                }),
            },
        );
    }

    SimulationController::new(nodes, drone_recv, topology, pool)
}

fn drone_options(
    config: &Config,
    nodes: &mut HashMap<NodeId, Node>,
    packets: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
    controller_send: Sender<DroneEvent>,
) -> Vec<DroneOptions> {
    config
        .drone
        .iter()
        .map(|drone| {
            // controller
            let (drone_send, controller_recv) = unbounded();
            nodes.insert(
                drone.id,
                Node {
                    packet_send: packets[&drone.id].0.clone(),
                    node_type: NodeType::Drone(NodeDrone { drone_send }),
                },
            );
            let controller_send = controller_send.clone();
            // packet
            let packet_recv = packets[&drone.id].1.clone();
            let packet_send = drone
                .connected_node_ids
                .iter()
                .cloned()
                .map(|id| (id, packets[&id].0.clone()))
                .collect();
            let id = drone.id;
            let pdr = drone.pdr;

            DroneOptions {
                id,
                controller_send,
                controller_recv,
                packet_recv,
                packet_send,
                pdr,
            }
        })
        .collect()
}

fn init_topology(config: &Config) -> Topology {
    let mut graph = Topology::new();

    for drone in config.drone.iter() {
        graph.add_node(drone.id);
    }
    for client in config.client.iter() {
        graph.add_node(client.id);
    }
    for server in config.server.iter() {
        graph.add_node(server.id);
    }

    for drone in config.drone.iter() {
        for neighbor_id in drone.connected_node_ids.iter() {
            graph.add_edge(drone.id, *neighbor_id, ());
        }
    }
    for client in config.client.iter() {
        for neighbor_id in client.connected_drone_ids.iter() {
            graph.add_edge(client.id, *neighbor_id, ());
        }
    }
    for server in config.server.iter() {
        for neighbor_id in server.connected_drone_ids.iter() {
            graph.add_edge(server.id, *neighbor_id, ());
        }
    }

    graph
}
