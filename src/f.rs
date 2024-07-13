use std::any::type_name_of_val;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

const THREADS: usize = 10;
const MESSAGES: usize = 100_000;

pub fn fn1() {
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
    println!("Arc {}", type_name_of_val(&counter));
    println!("Counter: {}", counter.load(Ordering::SeqCst));
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn fn2() {
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
                    while let Err(_) = tx.send(true) {}
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

pub fn fn3() {
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
                    while let Err(_) = tx.send(true) {}
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
