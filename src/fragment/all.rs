use super::fragment;
use crate::utils::topology;
use rayon::Scope;
use std::time::Duration;
use wg_2024::drone::Drone;

pub fn forward<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::forward::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
    fragment::forward::<D>(scope, &topology::STAR, timeout);
    fragment::forward::<D>(scope, &topology::BUTTERFLY, timeout);
    fragment::forward::<D>(scope, &topology::TREE, timeout);
    fragment::forward::<D>(scope, &topology::SUBNET_STAR, timeout);
    fragment::forward::<D>(scope, &topology::SUBNET_TRIANGLE, timeout);
}

pub fn avoid_crash<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::avoid_crash::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
    fragment::avoid_crash::<D>(scope, &topology::STAR, timeout);
    fragment::avoid_crash::<D>(scope, &topology::BUTTERFLY, timeout);
    fragment::avoid_crash::<D>(scope, &topology::TREE, timeout);
    fragment::avoid_crash::<D>(scope, &topology::SUBNET_STAR, timeout);
    fragment::avoid_crash::<D>(scope, &topology::SUBNET_TRIANGLE, timeout);
}

pub fn crash<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::crash::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn error_in_routing<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::error_in_routing::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn destination_is_drone<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::destination_is_drone::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn pdr<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::pdr::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn unexpected_recipient<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::unexpected_recipient::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}

pub fn dropped_packets_during_crash<D: Drone>(scope: &Scope, timeout: Duration) {
    fragment::dropped_packets_during_crash::<D>(scope, &topology::DOUBLE_CHAIN, timeout);
}
