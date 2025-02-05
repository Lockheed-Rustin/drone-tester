use super::controller;
use crate::utils::topology;
use rayon::Scope;
use std::time::Duration;
use wg_2024::drone::Drone;

pub fn packet_sent<D: Drone>(scope: &Scope, timeout: Duration) {
    controller::packet_sent::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
    controller::packet_sent::<D>(scope, &topology::STAR, timeout);
    controller::packet_sent::<D>(scope, &topology::BUTTERFLY, timeout);
    controller::packet_sent::<D>(scope, &topology::TREE, timeout);
    controller::packet_sent::<D>(scope, &topology::SUBNET_STAR, timeout);
    controller::packet_sent::<D>(scope, &topology::SUBNET_TRIANGLE, timeout);
}

pub fn packet_dropped<D: Drone>(scope: &Scope, timeout: Duration) {
    controller::packet_dropped::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn shortcut<D: Drone>(scope: &Scope, timeout: Duration) {
    controller::shortcut::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn no_neighbor_after_drop<D: Drone>(scope: &Scope, timeout: Duration) {
    controller::no_neighbor_after_drop::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn shortcut_packets_during_crash<D: Drone>(scope: &Scope, timeout: Duration) {
    controller::shortcut_packets_during_crash::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}
