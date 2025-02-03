use super::controller;
use crate::utils::topology;
use std::time::Duration;
use wg_2024::drone::Drone;

pub fn packet_sent<D: Drone>(timeout: Duration) {
    controller::packet_sent::<D>(&topology::DOUBLE_CHAIN, timeout);
    controller::packet_sent::<D>(&topology::STAR, timeout);
    controller::packet_sent::<D>(&topology::BUTTERFLY, timeout);
    controller::packet_sent::<D>(&topology::TREE, timeout);
    controller::packet_sent::<D>(&topology::SUBNET_STAR, timeout);
    controller::packet_sent::<D>(&topology::SUBNET_TRIANGLE, timeout);
}

pub fn packet_dropped<D: Drone>(timeout: Duration) {
    controller::packet_dropped::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn shortcut<D: Drone>(timeout: Duration) {
    controller::shortcut::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn no_neighbor_after_drop<D: Drone>(timeout: Duration) {
    controller::no_neighbor_after_drop::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn shortcut_packets_during_crash<D: Drone>(timeout: Duration) {
    controller::shortcut_packets_during_crash::<D>(&topology::DOUBLE_CHAIN, timeout);
}
