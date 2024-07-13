use futures::stream::FuturesUnordered;
use futures::StreamExt;

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

    let c = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(4)).await;
        panic!("test panic");
    });

    let mut tasks = FuturesUnordered::new();
    tasks.push(a);
    tasks.push(b);
    tasks.push(c);

    tasks.next().await.unwrap_or_else(|| unreachable!())?;

    println!("About to finish...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("Bye!");

    Ok(())
}
