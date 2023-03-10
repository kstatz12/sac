use screenshot::ScreenShot;
use tick::Scheduler;
use std::time::Duration;
use tokio::sync::mpsc;
use rand::Rng;
use rand::distributions::Uniform;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
#[cfg(unix)]
use signal_hook::{iterator::Signals, SIGTERM};
use ctrlc;



pub mod tick;
pub mod screenshot;
pub mod cfg;


#[tokio::main]
async fn main() {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();

   #[cfg(unix)]
    {
        let signals = Signals::new(&[SIGTERM]).unwrap();
        let r = running.clone();
        std::thread::spawn(move || {
            for sig in signals.forever() {
                match sig {
                    SIGTERM => {
                        println!("Received SIGTERM");
                        r.store(false, Ordering::SeqCst);
                        break;
                    }
                    _ => (),
                }
            }
        });
    }

    while running.load(Ordering::SeqCst) {
        let mut s = Scheduler::new();
        s.register(|| Box::new(ScreenShot::new()));

        let (mut tx, mut rx) = mpsc::channel(100);

        let delay = s.delay.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(configure_delay(delay))).await;
                tx.send(()).await;
            }
        });

        while let Some(_) = rx.recv().await {
           s.exec().await;
        }
    }
    //shutdown logic
}

fn configure_delay(base: u32) -> u64 {
    let variance = get_variance(20, 30);
    let d =  match get_direction(variance) {
        true => base + variance,
        false => base - variance ,
    };
    return (d * 60) as u64;
}

fn get_direction(n: u32) -> bool {
    return n % 2 == 0;
}

fn get_variance(l: u32, u: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(l, u);
    return rng.sample(range);
}
