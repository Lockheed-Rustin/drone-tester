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

pub enum Node {
    Drone {
        packet_send: Sender<Packet>,
        drone_send: Sender<DroneCommand>,
    },
    Host {
        packet_recv: Receiver<Packet>,
    },
}

// TODO: save topology
pub struct SimulationController {
    nodes: HashMap<NodeId, Node>,
    drone_recv: Receiver<DroneEvent>,
}

impl SimulationController {
    pub fn new(nodes: HashMap<NodeId, Node>, drone_recv: Receiver<DroneEvent>) -> Self {
        Self { nodes, drone_recv }
    }
}
