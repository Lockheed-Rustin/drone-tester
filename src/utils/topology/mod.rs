use lazy_static::lazy_static;
use wg_2024::config::Config;

const DOUBLE_CHAIN_STR: &str = include_str!("topologies/double-chain.toml");

fn parse_topology(topology_str: &str) -> Config {
    toml::from_str(&topology_str).unwrap()
}

lazy_static! {
    pub static ref DOUBLE_CHAIN: Config = parse_topology(DOUBLE_CHAIN_STR);
}
