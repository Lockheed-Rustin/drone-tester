use lazy_static::lazy_static;
use wg_2024::config::Config;
use wg_2024::network::NodeId;

/// THIS IS THE DEFAULT CLIENT ID IN EVERY TOPOLOGY.
/// I have chose this value to be 0.
pub const CID: NodeId = 0;
/// THIS IS THE DEFAULT SERVER ID EVERY TOPOLOGY.
/// I have chose this value to be 1
pub const SID: NodeId = 1;

const DOUBLE_CHAIN_STR: &str = include_str!("topologies/double-chain.toml");
const STAR_STR: &str = include_str!("topologies/star.toml");
const BUTTERFLY_STR: &str = include_str!("topologies/butterfly.toml");
const TREE_STR: &str = include_str!("topologies/tree.toml");
const SUBNET_STAR_STR: &str = include_str!("topologies/subnet-star.toml");
const SUBNET_TRIANGLE_STR: &str = include_str!("topologies/subnet-triangle.toml");

fn parse_topology(topology_str: &str) -> Config {
    toml::from_str(&topology_str).unwrap()
}

lazy_static! {
    pub static ref DOUBLE_CHAIN: Config = parse_topology(DOUBLE_CHAIN_STR);
    pub static ref STAR: Config = parse_topology(STAR_STR);
    pub static ref BUTTERFLY: Config = parse_topology(BUTTERFLY_STR);
    pub static ref TREE: Config = parse_topology(TREE_STR);
    pub static ref SUBNET_STAR: Config = parse_topology(SUBNET_STAR_STR);
    pub static ref SUBNET_TRIANGLE: Config = parse_topology(SUBNET_TRIANGLE_STR);
}
