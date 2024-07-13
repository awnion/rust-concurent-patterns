use std::thread::sleep;

use rust_concurent_patterns::f::fn1;
use rust_concurent_patterns::f::fn2;
use rust_concurent_patterns::f::fn3;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fn2();
    sleep(std::time::Duration::from_secs(2));
    fn3();
    sleep(std::time::Duration::from_secs(2));
    fn1();
    Ok(())
}
