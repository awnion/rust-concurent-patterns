use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

pub fn fn1() {
    let start = std::time::Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));

    thread::scope(|s| {
        for _ in 0..20 {
            let counter = counter.clone();
            s.spawn(move || {
                for _ in 0..1000000 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });
    println!("Counter: {}", counter.load(Ordering::SeqCst));
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn fn2() {
    let start = std::time::Instant::now();

    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let mut counter = 0usize;

    thread::scope(|s| {
        for _ in 0..20 {
            let tx = tx.clone();
            s.spawn(move || {
                for _ in 0..1000000 {
                    while let Err(_) = tx.send(true) {}
                }
                tx.send(false).unwrap()
            });
        }

        while rx.recv().unwrap() {
            counter += 1;
        }
    });

    println!("Counter: {}", counter);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn fn3() {
    let start = std::time::Instant::now();

    let (tx, rx) = crossbeam_channel::unbounded::<bool>();
    let mut counter = 0usize;

    thread::scope(|s| {
        for _ in 0..20 {
            let tx = tx.clone();
            s.spawn(move || {
                for _ in 0..1000000 {
                    while let Err(_) = tx.send(true) {}
                }
                tx.send(false).unwrap()
            });
        }

        while rx.recv().unwrap() {
            counter += 1;
        }
    });

    println!("Counter: {}", counter);
    println!("Time elapsed: {:?}", start.elapsed());
}
