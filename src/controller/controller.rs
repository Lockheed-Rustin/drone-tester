use crate::utils::{
    data, network, rand_node_in_route,
    topology::{CID, SID, UNKNOWN_ID},
};
use rayon::Scope;
use std::time::Duration;
use wg_2024::{config::Config, controller::DroneEvent, drone::Drone};

pub fn packet_sent<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let packet = data::test_fragment(route);
    // expected packet
    let mut expected = packet.clone();
    let hop_len = packet.routing_header.hops.len();
    // received events
    controller.send_packet(CID, packet);
    expected.routing_header.hop_index += 1;
    while let Ok(response) = controller.drone_recv.recv_timeout(timeout) {
        expected.routing_header.hop_index += 1;
        assert_eq!(DroneEvent::PacketSent(expected.clone()), response);
    }
    assert_eq!(expected.routing_header.hop_index, hop_len - 1);
}

pub fn packet_dropped<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let (pdr_id, pdr_idx) = rand_node_in_route(&route);
    let packet = data::test_fragment(route.clone());
    // expected packet
    let mut expected = packet.clone();
    // set pdr
    controller.set_pdr(pdr_id, 1.0);
    // received packet
    controller.send_packet(CID, packet);
    while let Ok(response) = controller.drone_recv.recv_timeout(timeout) {
        if let DroneEvent::PacketDropped(_) = response {
            expected.routing_header.hop_index = pdr_idx;
            assert_eq!(DroneEvent::PacketDropped(expected), response);
            return;
        }
    }
    panic!("no packet dropped received");
}

pub fn shortcut<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let mut controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let (crash_id, _) = rand_node_in_route(&route);
    let packet = data::test_ack(route.clone());
    // expected packet
    let mut expected = packet.clone();
    // crash drone
    controller.crash(crash_id);
    // received packet
    controller.send_packet(CID, packet);
    while let Ok(response) = controller.drone_recv.recv_timeout(timeout) {
        if let DroneEvent::ControllerShortcut(ref r) = response {
            // hop_index value after shortcut is not defined in the protocol
            // set it equal to response to ignore it
            expected.routing_header.hop_index = r.routing_header.hop_index;
            assert_eq!(DroneEvent::ControllerShortcut(expected), response);
            return;
        }
    }
    panic!("no controller shortcut received");
}

pub fn no_neighbor_after_drop<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(scope, config);

    // packet
    let mut route = controller.route(CID, SID);
    let drone_id = route.hops[1];
    route.hops[0] = UNKNOWN_ID;
    route.hops[1] = UNKNOWN_ID;
    let mut packet = data::test_fragment(route);
    packet.routing_header.hop_index += 1;
    // expected packet
    let mut expected = packet.clone();
    // received packet
    controller.send_packet_to(drone_id, packet);
    let response = controller.drone_recv.recv_timeout(timeout).unwrap();

    if let DroneEvent::ControllerShortcut(ref r) = response {
        // hop_index value after shortcut is not defined in the protocol
        // set it equal to response to ignore it
        expected.routing_header.hop_index = r.routing_header.hop_index;
    }
    assert_eq!(DroneEvent::ControllerShortcut(expected), response);
}

pub fn shortcut_packets_during_crash<D: Drone>(scope: &Scope, config: &Config, timeout: Duration) {
    let mut controller = network::init_network::<D>(scope, config);

    // packet
    let route = controller.route(CID, SID);
    let (crash_id, _) = rand_node_in_route(&route);
    let packet = data::test_ack(route);
    // expected packet
    let mut expected = packet.clone();
    // crash drone
    controller.send_crash(crash_id);
    // received packet
    const SEND_COUNT: usize = 10;
    for _ in 0..SEND_COUNT {
        controller.send_packet(CID, packet.clone());
    }
    let mut i = 0;
    while let Ok(response) = controller.drone_recv.recv_timeout(timeout) {
        if let DroneEvent::ControllerShortcut(ref r) = response {
            i += 1;
            // hop_index value after shortcut is not defined in the protocol
            // set it equal to response to ignore it
            expected.routing_header.hop_index = r.routing_header.hop_index;
            assert_eq!(DroneEvent::ControllerShortcut(expected.clone()), response);
        }
    }
    assert_eq!(i, SEND_COUNT);
}
