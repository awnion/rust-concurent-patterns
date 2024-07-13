use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

use tokio::sync::oneshot;

const THREADS: usize = 20;
const MESSAGES: usize = 1000_000;

pub fn arc_atomic_counter() {
    let start = std::time::Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));

    thread::scope(|s| {
        for _ in 0..THREADS {
            let counter = counter.clone();
            s.spawn(move || {
                for _ in 0..MESSAGES {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });

    println!("Counter: {}", counter.load(Ordering::SeqCst));
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn std_sync_mpsc_channel() {
    let start = std::time::Instant::now();

    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let mut counter = 0usize;

    thread::scope(|s| {
        let counter = &mut counter;
        s.spawn(move || {
            while rx.recv().unwrap() {
                *counter += 1;
            }
        });
        let mut pool = Vec::new();
        for _ in 0..THREADS {
            let tx = tx.clone();
            pool.push(s.spawn(move || {
                for _ in 0..MESSAGES {
                    let _ = tx.send(true);
                }
            }));
        }

        for t in pool {
            t.join().unwrap();
        }

        while let Err(_) = tx.send(false) {}
    });

    println!("Counter: {}", counter);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn crossbeam_unbounded() {
    let start = std::time::Instant::now();

    let (tx, rx) = crossbeam_channel::unbounded::<bool>();
    let mut counter = 0usize;

    thread::scope(|s| {
        let counter = &mut counter;
        s.spawn(move || {
            while rx.recv().unwrap() {
                *counter += 1;
            }
        });
        let mut pool = Vec::new();
        for _ in 0..THREADS {
            let tx = tx.clone();
            pool.push(s.spawn(move || {
                for _ in 0..MESSAGES {
                    let _ = tx.send(true);
                }
            }));
        }

        for t in pool {
            t.join().unwrap();
        }

        while let Err(_) = tx.send(false) {}
    });

    println!("Counter: {}", counter);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub async fn async_std_sync_mpsc_channel() {
    let start = std::time::Instant::now();

    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let (counter_tx, counter_rx) = oneshot::channel();

    tokio::spawn(async move {
        let mut counter = 0usize;
        while rx.recv().unwrap() {
            counter += 1;
        }
        counter_tx.send(counter).unwrap();
    });

    let mut pool = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        pool.push(tokio::spawn(async move {
            for _ in 0..MESSAGES {
                let _ = tx.send(true);
            }
        }));
    }

    for t in pool {
        let _ = t.await;
    }

    while let Err(_) = tx.send(false) {}

    let counter = counter_rx.await.unwrap();

    println!("Counter: {}", counter);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub async fn async_crossbeam_unbounded() {
    let start = std::time::Instant::now();

    let (tx, rx) = crossbeam_channel::unbounded::<bool>();
    let (counter_tx, counter_rx) = oneshot::channel();

    tokio::spawn(async move {
        let mut counter = 0usize;
        while rx.recv().unwrap() {
            counter += 1;
        }
        counter_tx.send(counter).unwrap();
    });

    let mut pool = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        pool.push(tokio::spawn(async move {
            for _ in 0..MESSAGES {
                let _ = tx.send(true);
            }
        }));
    }

    for t in pool {
        let _ = t.await;
    }

    while let Err(_) = tx.send(false) {}

    let counter = counter_rx.await.unwrap();

    println!("Counter: {}", counter);
    println!("Time elapsed: {:?}", start.elapsed());
}
