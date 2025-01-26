use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use petgraph::algo::astar;
use petgraph::prelude::UnGraphMap;
use std::{collections::HashMap, time::Duration};
use wg_2024::{
    controller::{DroneCommand, DroneEvent},
    network::{NodeId, SourceRoutingHeader},
    packet::{Packet, PacketType},
};

pub struct Node {
    pub packet_send: Sender<Packet>,
    pub node_type: NodeType,
}

pub struct NodeDrone {
    pub drone_send: Sender<DroneCommand>,
}

pub struct NodeHost {
    pub packet_recv: Receiver<Packet>,
    pub packet_send: HashMap<NodeId, Sender<Packet>>,
}

pub enum NodeType {
    Drone(NodeDrone),
    Host(NodeHost),
}

pub type Topology = UnGraphMap<NodeId, ()>;

pub struct SimulationController {
    pub nodes: HashMap<NodeId, Node>,
    pub drone_recv: Receiver<DroneEvent>,

    pub topology: Topology,
}

impl SimulationController {
    pub fn new(
        nodes: HashMap<NodeId, Node>,
        drone_recv: Receiver<DroneEvent>,
        topology: Topology,
    ) -> Self {
        Self {
            nodes,
            drone_recv,
            topology,
        }
    }

    pub fn route(&self, a: NodeId, b: NodeId) -> SourceRoutingHeader {
        let (_, path) = astar::astar(&self.topology, a, |g| g == b, |_| 1, |_| 0).unwrap();
        SourceRoutingHeader {
            hop_index: 0,
            hops: path,
        }
    }

    pub fn send_packet(&self, a: NodeId, mut packet: Packet) {
        let host = self.get_host(a).unwrap();
        match packet.pack_type {
            PacketType::FloodRequest(_) => {
                for (_, packet_send) in host.packet_send.iter() {
                    packet_send.send(packet.clone()).unwrap();
                }
            }
            _ => {
                let next_hop = packet.routing_header.hops[1];
                packet.routing_header.hop_index += 1;
                host.packet_send[&next_hop].send(packet).unwrap();
            }
        }
    }

    pub fn send_packet_to(&self, a: NodeId, packet: Packet) {
        self.nodes[&a].packet_send.send(packet).unwrap();
    }

    pub fn recv_packet_timeout(
        &self,
        a: NodeId,
        timeout: Duration,
    ) -> Result<Packet, RecvTimeoutError> {
        let host = self.get_host(a).unwrap();
        host.packet_recv.recv_timeout(timeout)
    }

    pub fn get_host(&self, a: NodeId) -> Option<&NodeHost> {
        match &self.nodes.get(&a)?.node_type {
            NodeType::Host(host) => Some(host),
            _ => None,
        }
    }

    pub fn get_drone(&self, a: NodeId) -> Option<&NodeDrone> {
        match &self.nodes.get(&a)?.node_type {
            NodeType::Drone(drone) => Some(drone),
            _ => None,
        }
    }

    pub fn crash(&mut self, a: NodeId) {
        let drone = self.get_drone(a).unwrap();
        drone.drone_send.send(DroneCommand::Crash).unwrap();
        let n = self.topology.neighbors(a);
        for neighbor in n {
            match self.nodes.get_mut(&neighbor).unwrap().node_type {
                NodeType::Drone(ref drone) => drone
                    .drone_send
                    .send(DroneCommand::RemoveSender(a))
                    .unwrap(),
                NodeType::Host(ref mut host) => _ = host.packet_send.remove(&a),
            }
        }
        self.nodes.remove(&a);
        self.topology.remove_node(a);
    }

    pub fn send_crash(&mut self, a: NodeId) {
        let drone = self.get_drone(a).unwrap();
        drone.drone_send.send(DroneCommand::Crash).unwrap();
    }

    pub fn set_pdr(&self, a: NodeId, pdr: f32) {
        let drone = self.get_drone(a).unwrap();
        drone
            .drone_send
            .send(DroneCommand::SetPacketDropRate(pdr))
            .unwrap();
    }
}
