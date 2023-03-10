use std::boxed::Box;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};
use std::io;

#[async_trait]
pub trait Job {
    fn exec(&mut self) -> io::Result<()>;
}

pub struct Scheduler {
    jobs: Vec<Box<dyn Job>>,
    pub delay: u32,
}

impl Scheduler {
    pub fn register(&mut self, f: fn() -> Box<dyn Job>) {
        self.jobs.push(f());
    }

    pub fn new() -> Self {
        Scheduler {
            jobs: Vec::new(),
            delay: 5,
        }
    }

    pub async fn exec(&mut self) {
        for j in &mut self.jobs {
            j.exec().await;
        }
    }
}
