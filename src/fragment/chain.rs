use crate::utils::{data, network, topology};
use wg_2024::{drone::Drone, packet::NackType};

pub fn double_chain<T: Drone>() {
    let controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let mut packet = controller.with_route(0, 2, data::test_fragment());
    controller.send_packet(0, packet.clone());

    let response = controller.recv_packet(2);

    packet.routing_header.hop_index = 6;
    assert_eq!(packet, response);
}

pub fn crash_double_chain<T: Drone>() {
    let mut controller = network::init_network::<T>(&topology::DOUBLE_CHAIN);

    let mut packet = data::test_fragment();
    let route = controller.route(0, 2);
    let crash_idx = 2;
    let crash_id = route.hops[crash_idx];
    let hops = route
        .hops
        .iter()
        .cloned()
        .take(crash_idx)
        .rev()
        .collect::<Vec<_>>();
    packet.routing_header = route;
    controller.crash(crash_id);

    controller.send_packet(0, packet);

    let response = controller.recv_packet(0);
    let expected = data::test_nack(hops, NackType::ErrorInRouting(crash_id));
    assert_eq!(expected, response);
}
