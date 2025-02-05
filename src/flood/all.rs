use super::flood::assert_topology;
use crate::utils::topology;
use rayon::Scope;
use std::time::Duration;
use wg_2024::drone::Drone;

pub fn double_chain<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::DOUBLE_CHAIN, timeout, true);
}

pub fn double_chain_no_initiator<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::DOUBLE_CHAIN, timeout, false);
}

pub fn star<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::STAR, timeout, true);
}

pub fn star_no_initiator<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::STAR, timeout, false);
}

pub fn butterfly<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::BUTTERFLY, timeout, true);
}

pub fn butterfly_no_initiator<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::BUTTERFLY, timeout, false);
}

pub fn tree<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::TREE, timeout, true);
}

pub fn tree_no_initiator<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::TREE, timeout, false);
}

pub fn subnet_star<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::SUBNET_STAR, timeout, true);
}

pub fn subnet_star_no_initiator<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::SUBNET_STAR, timeout, false);
}

pub fn subnet_triangle<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::SUBNET_TRIANGLE, timeout, true);
}

pub fn subnet_triangle_no_initiator<D: Drone>(scope: &Scope, timeout: Duration) {
    assert_topology::<D>(scope, &topology::SUBNET_TRIANGLE, timeout, false);
}
