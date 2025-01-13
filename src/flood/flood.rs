use crate::flood::assert_topology;
use crate::utils::topology::{BUTTERFLY, DOUBLE_CHAIN, STAR, SUBNET_STAR, SUBNET_TRIANGLE, TREE};
use std::time::Duration;
use wg_2024::drone::Drone;

pub fn double_chain<T: Drone>(timeout: Duration) {
    assert_topology::<T>(&DOUBLE_CHAIN, timeout);
}

pub fn star<T: Drone>(timeout: Duration) {
    assert_topology::<T>(&STAR, timeout);
}

pub fn butterfly<T: Drone>(timeout: Duration) {
    assert_topology::<T>(&BUTTERFLY, timeout);
}

pub fn tree<T: Drone>(timeout: Duration) {
    assert_topology::<T>(&TREE, timeout);
}

pub fn subnet_star<T: Drone>(timeout: Duration) {
    assert_topology::<T>(&SUBNET_STAR, timeout);
}

pub fn subnet_triangle<T: Drone>(timeout: Duration) {
    assert_topology::<T>(&SUBNET_TRIANGLE, timeout);
}
