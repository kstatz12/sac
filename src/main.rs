pub mod cfg;
pub mod device;
pub mod dispatch;
pub mod screenshot;
pub mod helpers;

use std::{io::Error, borrow::Borrow};

use dispatch::Dispatch;
use screenshot::ScreenShot;
use tokio::signal::unix::SignalKind;

async fn run() {
    let mut d = Dispatch::new();
    let scrn = ScreenShot::new();

    d.send_repeated(String::from("ScreenShot"), 5);


    d.register_handler(String::from("ScreenShot"), move || {
        let s = scrn.borrow();
        s.capture();
    });
}

#[tokio::main]
async fn main() {
    let ctrl_c = CtrlC::new().expect("failed to create CtrlC handler");

    let mut ctrl_c_stream = signal(SignalKind::interrupt()).expect("failed to create SIGINT stream");
    let ctrl_c_stream = ctrl_c_stream.map(|_| CtrlCEvent::CtrlC);

    tokio::select! {
        _ = ctrl_c_stream.select_next_some() => {
            // Handle Ctrl-C event
            println!("Ctrl-C detected, exiting gracefully...");
            // Do any cleanup or other actions needed to exit gracefully here
        }
        _ = run() => {
            // This branch will execute when the application finishes running
            println!("Application finished running");
        }
    }
}
