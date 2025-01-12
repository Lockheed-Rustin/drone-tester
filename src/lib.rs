use crossbeam_channel::bounded;
use std::thread;
use std::time::Duration;

pub mod flood;
pub mod fragment;
pub mod general;
mod test_drones;
mod utils;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(100);

pub fn with_timeout<F>(f: F, timeout: Duration)
where
    F: FnOnce() + Send + 'static,
{
    let (tx, rx) = bounded(0);
    thread::spawn(move || {
        f();
        _ = tx.send(());
    });
    if let Err(_) = rx.recv_timeout(timeout) {
        panic!("function time out");
    }
}

// TODO: remove
test_drones!(
    lockheedrustin_drone::LockheedRustin
    rusty_drones::RustyDrone
);
