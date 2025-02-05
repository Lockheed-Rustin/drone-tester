//! tests in this module are usually composed of 6 different steps
//! 1. the network is initialized
//! 2. the packet to be sent is crated
//! 3. some actions are sent to the simulation controller
//! 4. the expected packet is created
//! 5. the packet is sent and the response is received
//! 6. the expected packet and the response are created

use crate::utils::topology::{CID, SID, UNKNOWN_ID};
use crate::utils::{data, network, rand_node_in_route};
use rayon::Scope;
use std::time::Duration;
use wg_2024::{config::Config, drone::Drone, packet::NackType};

pub fn forward<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let packet = data::test_fragment(route);
    // expected packet
    let mut expected = packet.clone();
    let hop_len = packet.routing_header.hops.len();
    expected.routing_header.hop_index = hop_len - 1;
    // received packet
    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(SID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn avoid_crash<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let mut controller = network::init_network::<D>(scope, config);

    // crash drone
    let route = controller.route(CID, SID);
    let (crash_id, _) = rand_node_in_route(&route);
    controller.crash(crash_id);
    // packet
    let route = controller.route(CID, SID);
    let packet = data::test_fragment(route);
    // expected packet
    let mut expected = packet.clone();
    let hop_len = packet.routing_header.hops.len();
    expected.routing_header.hop_index = hop_len - 1;
    // received packet
    controller.send_packet(CID, packet.clone());
    let response = controller.recv_packet_timeout(SID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn crash<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let mut controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let (crash_id, crash_idx) = rand_node_in_route(&route);
    let packet = data::test_fragment(route.clone());
    // expected packet
    let expected = data::test_nack(route, crash_idx - 1, NackType::ErrorInRouting(crash_id));
    // crash drone
    controller.crash(crash_id);
    // received packet
    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn error_in_routing<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let mut route = controller.route(CID, SID);
    let (_, err_idx) = rand_node_in_route(&route);
    route.hops[err_idx] = UNKNOWN_ID;
    let packet = data::test_fragment(route.clone());
    // expected packet
    let expected = data::test_nack(route, err_idx - 1, NackType::ErrorInRouting(UNKNOWN_ID));
    // received packet
    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn destination_is_drone<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let mut route = controller.route(CID, SID);
    _ = route.hops.pop();
    let packet = data::test_fragment(route.clone());
    // expected packet
    let hop_len = packet.routing_header.hops.len();
    let expected = data::test_nack(route, hop_len - 1, NackType::DestinationIsDrone);
    // received packet
    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn pdr<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let (pdr_id, pdr_idx) = rand_node_in_route(&route);
    let packet = data::test_fragment(route.clone());
    // set pdr
    controller.set_pdr(pdr_id, 1.0);
    // expected packet
    let expected = data::test_nack(route, pdr_idx, NackType::Dropped);
    // received packet
    controller.send_packet(CID, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn unexpected_recipient<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let drone_id = route.hops[1];
    let mut modified_route = route.clone();
    modified_route.hops[1] = UNKNOWN_ID;
    let mut packet = data::test_fragment(modified_route);
    packet.routing_header.hop_index += 1;
    // expected packet
    let expected = data::test_nack(route, 1, NackType::UnexpectedRecipient(drone_id));
    // received packet
    controller.send_packet_to(drone_id, packet);
    let response = controller.recv_packet_timeout(CID, timeout).unwrap();

    assert_eq!(expected, response);
}

pub fn dropped_packets_during_crash<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let mut controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let (crash_id, crash_idx) = rand_node_in_route(&route);
    let packet = data::test_fragment(route.clone());
    // expected packet
    let expected = data::test_nack(route, crash_idx, NackType::ErrorInRouting(crash_id));
    // crash drone
    controller.send_crash(crash_id);
    // received packet
    const SEND_COUNT: usize = 10;
    for _ in 0..SEND_COUNT {
        controller.send_packet(CID, packet.clone());
    }
    for _ in 0..SEND_COUNT {
        let response = controller.recv_packet_timeout(CID, timeout).unwrap();
        assert_eq!(expected, response);
    }
}
