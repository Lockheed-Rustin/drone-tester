use std::time::Duration;

pub mod controller;
pub mod flood;
pub mod fragment;
mod test_drones;
mod utils;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(250);
