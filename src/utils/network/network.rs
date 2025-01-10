use super::controller::{Node, SimulationController};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use wg_2024::{
    config::Config,
    controller::{DroneCommand, DroneEvent},
    drone::Drone,
    network::NodeId,
    packet::Packet,
};

pub fn init_network<T: Drone>(config: &Config) -> SimulationController {
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

    init_drones::<T>(&config, &mut nodes, &packets, drone_send.clone());

    for client in config.client.iter() {
        nodes.insert(
            client.id,
            Node::Host {
                packet_recv: packets[&client.id].1.clone(),
            },
        );
    }
    for server in config.server.iter() {
        nodes.insert(
            server.id,
            Node::Host {
                packet_recv: packets[&server.id].1.clone(),
            },
        );
    }

    SimulationController::new(nodes, drone_recv)
}

fn init_drones<T: Drone>(
    config: &Config,
    nodes: &mut HashMap<NodeId, Node>,
    packets: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
    controller_send: Sender<DroneEvent>,
) {
    for drone in config.drone.iter() {
        // controller
        let (drone_send, controller_recv) = unbounded();
        nodes.insert(
            drone.id,
            Node::Drone {
                packet_send: packets[&drone.id].0.clone(),
                drone_send,
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
        let drone_id = drone.id;
        let drone_pdr = drone.pdr;

        thread::spawn(move || {
            T::new(
                drone_id,
                controller_send,
                controller_recv,
                packet_recv,
                packet_send,
                drone_pdr,
            )
            .run();
        });
    }
}
