use crate::utils::{
    data, network, rand_node_in_route,
    topology::{CID, SID},
};
use std::time::Duration;
use wg_2024::{config::Config, controller::DroneEvent, drone::Drone};

pub fn packet_sent<D: Drone>(config: &Config, timeout: Duration) {
    let controller = network::init_network::<D>(config);

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

pub fn packet_dropped<D: Drone>(config: &Config, timeout: Duration) {
    let mut controller = network::init_network::<D>(config);

    // packet
    let route = controller.route(CID, SID);
    let (crash_id, crash_idx) = rand_node_in_route(&route);
    let packet = data::test_fragment(route.clone());
    // expected packet
    let mut expected = packet.clone();
    // crash drone
    controller.crash(crash_id);
    // received packet
    controller.send_packet(CID, packet);
    while let Ok(response) = controller.drone_recv.recv_timeout(timeout) {
        if let DroneEvent::PacketDropped(_) = response {
            expected.routing_header.hop_index = crash_idx;
            assert_eq!(DroneEvent::PacketDropped(expected.clone()), response);
        }
    }
}
