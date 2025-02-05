use super::fragment;
use crate::utils::topology;
use std::time::Duration;
use wg_2024::drone::Drone;

pub fn forward<D: Drone>(timeout: Duration) {
    fragment::forward::<D>(&topology::DOUBLE_CHAIN, timeout);
    fragment::forward::<D>(&topology::STAR, timeout);
    fragment::forward::<D>(&topology::BUTTERFLY, timeout);
    fragment::forward::<D>(&topology::TREE, timeout);
    fragment::forward::<D>(&topology::SUBNET_STAR, timeout);
    fragment::forward::<D>(&topology::SUBNET_TRIANGLE, timeout);
}

pub fn avoid_crash<D: Drone>(timeout: Duration) {
    fragment::avoid_crash::<D>(&topology::DOUBLE_CHAIN, timeout);
    fragment::avoid_crash::<D>(&topology::STAR, timeout);
    fragment::avoid_crash::<D>(&topology::BUTTERFLY, timeout);
    fragment::avoid_crash::<D>(&topology::TREE, timeout);
    fragment::avoid_crash::<D>(&topology::SUBNET_STAR, timeout);
    fragment::avoid_crash::<D>(&topology::SUBNET_TRIANGLE, timeout);
}

pub fn crash<D: Drone>(timeout: Duration) {
    fragment::crash::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn error_in_routing<D: Drone>(timeout: Duration) {
    fragment::error_in_routing::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn destination_is_drone<D: Drone>(timeout: Duration) {
    fragment::destination_is_drone::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn pdr<D: Drone>(timeout: Duration) {
    fragment::pdr::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn unexpected_recipient<D: Drone>(timeout: Duration) {
    fragment::unexpected_recipient::<D>(&topology::DOUBLE_CHAIN, timeout);
}

pub fn dropped_packets_during_crash<D: Drone>(timeout: Duration) {
    fragment::dropped_packets_during_crash::<D>(&topology::DOUBLE_CHAIN, timeout);
}
