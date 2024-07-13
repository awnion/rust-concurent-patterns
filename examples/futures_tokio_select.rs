#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init some data
    let test = 1;
    let test2 = 2;

    let a = tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("Hello {}", test);
        }
    });

    let b = tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("world! {}", test2);
        }
    });

    // let c = tokio::spawn(async move {
    //     tokio::time::sleep(std::time::Duration::from_secs(4)).await;
    //     panic!("test panic");
    // });

    let c = tokio::task::spawn_blocking(move || {
        let thread_handler = std::thread::Builder::new()
            .name("panicing thread".into())
            .spawn(move || {
                panic!("blocking test");
            })
            .expect("Failed to spawn thread");
        thread_handler.join().expect("expect to join the thread");
    });

    tokio::select! {
        v = a => v?,
        v = b => v?,
        v = c => v?,
    }

    println!("About to finish...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("Bye!");

    Ok(())
}
