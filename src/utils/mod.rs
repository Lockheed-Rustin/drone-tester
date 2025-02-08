use crossbeam_channel::{Receiver, Sender};
use rand::Rng;
use std::collections::HashMap;
use wg_2024::{
    controller::{DroneCommand, DroneEvent},
    network::{NodeId, SourceRoutingHeader},
    packet::Packet,
};

pub mod controller;
pub mod data;
pub mod network;
pub mod topology;

pub struct DroneOptions {
    id: NodeId,
    controller_send: Sender<DroneEvent>,
    controller_recv: Receiver<DroneCommand>,
    packet_recv: Receiver<Packet>,
    packet_send: HashMap<NodeId, Sender<Packet>>,
    pdr: f32,
}

pub fn rand_node_in_route(route: &SourceRoutingHeader) -> (NodeId, usize) {
    let hop_len = route.hops.len();
    assert!(hop_len >= 5);
    // exclude first and last drone
    let rand_idx = rand::thread_rng().gen_range(2..hop_len - 2);
    (route.hops[rand_idx], rand_idx)
}

pub fn reverse_route(route: SourceRoutingHeader, drop_idx: usize) -> SourceRoutingHeader {
    let hops = route
        .hops
        .into_iter()
        .take(drop_idx + 1)
        .rev()
        .collect::<Vec<_>>();
    let hop_index = hops.len() - 1;
    SourceRoutingHeader { hops, hop_index }
}
