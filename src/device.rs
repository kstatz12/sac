use std::{time::{Instant, SystemTime, UNIX_EPOCH}, sync::{Mutex, Arc}, borrow::BorrowMut};

use device_query::{MousePosition, DeviceState, DeviceEvents};
use tokio::sync::mpsc;

use crate::common::{Startable, Stoppable};

pub struct MouseService {
    history: Vec<Point>
}

impl MouseService
{
    pub fn new() -> Self {
        MouseService { history: Vec::new() }
    }

    async fn capture(&mut self) {
        let ds = DeviceState::new();
        let (tx, mut rx) = mpsc::channel::<MousePosition>(1000);
        let _guard = ds.on_mouse_move(move |p| {
            let tx_clone = tx.clone();
            let p_clone = p.clone();
            tokio::spawn(async move {
                tx_clone.send(p_clone).await.unwrap();
            });
        });

        while let Some(pos) = rx.recv().await {
            self.history.push(Point::adapt(&pos));
        }
    }
}

impl Startable for MouseService {
    fn start(&mut self) {
        self.capture();
    }
}

impl Stoppable for MouseService {
    fn stop(&mut self, on_stop: fn(c: &mut MouseService)) {
        on_stop(self);
    }
}

struct Point {
    x: i32,
    y: i32,
    ts: String
}

fn ts_with_nanos() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_nanos().to_string();
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
       return Point { x, y, ts: ts_with_nanos() };
    }

    fn adapt(pos: &MousePosition) -> Point {
        let (x, y) = *pos;
        return Point::new(x, y);
    }
}
