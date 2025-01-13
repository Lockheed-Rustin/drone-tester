use std::time::Duration;

pub mod flood;
pub mod fragment;
pub mod general;
mod test_drones;
mod utils;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(250);

// TODO: remove
test_drones!(
    lockheedrustin_drone::LockheedRustin
    rusty_drones::RustyDrone
);
