use std::time::Duration;

use crate::utils::topology::{CID, SID};
use crate::utils::{data, network, topology};
use wg_2024::{drone::Drone, packet::NackType};

pub fn receive<T: Drone>(timeout: Duration) {
    let controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let route = controller.route(CID, SID);
    let hop_len = route.hops.len();
    let mut packet = data::test_fragment();
    packet.routing_header = route;

    controller.send_packet(CID, packet.clone());
    let response = controller.recv_packet_timeout(SID, timeout).unwrap();

    packet.routing_header.hop_index = hop_len - 1;
    assert_eq!(packet, response);
}

pub fn avoid_crash<T: Drone>(timeout: Duration) {
    let mut controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let route = controller.route(CID, SID);
    let hop_len = route.hops.len();
    let crash_idx = hop_len / 2;
    let crash_id = route.hops[crash_idx];
    controller.crash(crash_id);

    let route = controller.route(CID, SID);
    let hop_len = route.hops.len();
    let mut packet = data::test_fragment();
    packet.routing_header = route;

    controller.send_packet(CID, packet.clone());
    let response = controller.recv_packet_timeout(SID, timeout).unwrap();

    packet.routing_header.hop_index = hop_len - 1;
    assert_eq!(packet, response);
}

pub fn crash<T: Drone>(timeout: Duration) {
    let mut controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let route = controller.route(CID, SID);
    let hop_len = route.hops.len();
    let crash_idx = hop_len / 2;
    let crash_id = route.hops[crash_idx];
    let hops = route
        .hops
        .iter()
        .cloned()
        .take(crash_idx)
        .rev()
        .collect::<Vec<_>>();
    let mut packet = data::test_fragment();
    packet.routing_header = route;
    controller.crash(crash_id);

    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    let expected = data::test_nack(hops, NackType::ErrorInRouting(crash_id));
    assert_eq!(expected, response);
}

pub fn error_in_routing<T: Drone>(timeout: Duration) {
    let controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let mut route = controller.route(CID, SID);
    let hop_len = route.hops.len();
    let err_idx = hop_len / 2;
    route.hops[err_idx] = route.hops[err_idx + 1];
    let err_id = route.hops[err_idx];
    let hops = route
        .hops
        .iter()
        .cloned()
        .take(err_idx)
        .rev()
        .collect::<Vec<_>>();
    let mut packet = data::test_fragment();
    packet.routing_header = route;

    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    let expected = data::test_nack(hops, NackType::ErrorInRouting(err_id));
    assert_eq!(expected, response);
}

pub fn error_destination_is_drone<T: Drone>(timeout: Duration) {
    let controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let mut route = controller.route(CID, SID);
    _ = route.hops.pop();
    let hops = route.hops.iter().cloned().rev().collect::<Vec<_>>();
    let mut packet = data::test_fragment();
    packet.routing_header = route;

    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    let expected = data::test_nack(hops, NackType::DestinationIsDrone);
    assert_eq!(expected, response);
}

pub fn pdr<T: Drone>(timeout: Duration) {
    let controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let route = controller.route(CID, SID);
    let hop_len = route.hops.len();
    let pdr_idx = hop_len / 2;
    let pdr_id = route.hops[pdr_idx];
    controller.set_pdr(pdr_id, 1.0);
    let hops = route
        .hops
        .iter()
        .cloned()
        .take(pdr_idx + 1)
        .rev()
        .collect::<Vec<_>>();
    let mut packet = data::test_fragment();
    packet.routing_header = route;

    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    let expected = data::test_nack(hops, NackType::Dropped);
    assert_eq!(expected, response);
}
