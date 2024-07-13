use std::any::type_name_of_val;
use std::thread::sleep;

use rust_concurent_patterns::f::arc_atomic_counter;
use rust_concurent_patterns::f::crossbeam_unbounded;
use rust_concurent_patterns::f::std_sync_mpsc_channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("------------");
    sleep(std::time::Duration::from_secs(1));
    println!("------------");

    println!("{}", type_name_of_val(&std_sync_mpsc_channel));
    std_sync_mpsc_channel();

    println!("------------");
    sleep(std::time::Duration::from_secs(1));
    println!("------------");

    println!("{}", type_name_of_val(&crossbeam_unbounded));
    crossbeam_unbounded();

    println!("------------");
    sleep(std::time::Duration::from_secs(1));
    println!("------------");

    println!("{}", type_name_of_val(&arc_atomic_counter));
    arc_atomic_counter();

    Ok(())
}
