mod flood;

pub use flood::*;
use wg_2024::network::NodeId;

use crate::utils::controller::SimulationController;
use crate::utils::topology::CID;
use crate::utils::{data, network};
use std::collections::HashSet;
use std::time::Duration;
use wg_2024::config::Config;
use wg_2024::drone::Drone;
use wg_2024::packet::PacketType;

/// assumes the graph is connected
pub fn assert_topology<T: Drone>(config: &Config, timeout: Duration) {
    let controller = network::init_network::<T>(config);

    let flood = data::test_flood_request(0, CID, true);
    controller.send_packet(CID, flood);

    let result = build_drone_topology(&controller, timeout);
    let expected = build_drone_topology_from_config(config);

    assert_eq!(expected, result);
}

fn build_drone_topology(
    controller: &SimulationController,
    timeout: Duration,
) -> Vec<(NodeId, NodeId)> {
    let mut edges = HashSet::new();

    while let Ok(packet) = controller.recv_packet_timeout(CID, timeout) {
        if let PacketType::FloodResponse(ref flood_res) = packet.pack_type {
            let path = flood_res
                .path_trace
                .iter()
                .cloned()
                .map(|(id, _)| id)
                .skip(1);
            let connections = path
                .clone()
                .zip(path.skip(1))
                .map(|(a, b)| (a, b).min((b, a)));

            edges.extend(connections);
        }
    }

    let mut edges = edges.into_iter().collect::<Vec<_>>();
    edges.sort();
    edges
}

fn build_drone_topology_from_config(config: &Config) -> Vec<(NodeId, NodeId)> {
    let drone_ids = config
        .drone
        .iter()
        .map(|drone| drone.id)
        .collect::<HashSet<_>>();
    let mut edges = config
        .drone
        .iter()
        .flat_map(|drone| {
            drone
                .connected_node_ids
                .iter()
                .cloned()
                .filter_map(|neighbor_id| {
                    drone_ids
                        .contains(&neighbor_id)
                        .then(|| (drone.id, neighbor_id))
                })
        })
        .map(|(a, b)| (a, b).min((b, a)))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    edges.sort();
    edges
}
