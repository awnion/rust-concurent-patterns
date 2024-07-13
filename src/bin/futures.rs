use std::thread::sleep;

use rust_concurent_patterns::f::arc_atomic_counter;
use rust_concurent_patterns::f::crossbeam_unbounded;
use rust_concurent_patterns::f::std_sync_mpsc_channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std_sync_mpsc_channel();

    println!("------------");
    sleep(std::time::Duration::from_secs(1));
    println!("------------");

    crossbeam_unbounded();

    println!("------------");
    sleep(std::time::Duration::from_secs(1));
    println!("------------");

    arc_atomic_counter();

    Ok(())
}
