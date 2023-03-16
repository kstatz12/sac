use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::mpsc;

pub struct Dispatch {
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
    handlers: HashMap<String, fn()>,
}

impl Dispatch {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<String>(100);
        return Dispatch {
            sender: tx,
            receiver: rx,
            handlers: HashMap::new(),
        };
    }

    pub fn send_repeated(&self, key: String, delay: u64) {
        let key = Arc::new(key);
        let sender_clone = self.sender.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(delay)).await;
                let key_clone = key.clone();
                sender_clone.send(key_clone.to_string()).await.unwrap();
            }
        });
    }

    //making the borrow checker gods happy
    pub fn send(&self, key: String) {
        let key = Arc::new(key);
        let sender = self.sender.clone();
        tokio::spawn(async move {
            let kc = key.clone();
            sender.send(kc.to_string()).await.unwrap();
        });
    }

    pub fn register_handler(&mut self, key: String, f: fn()) {
        self.handlers.insert(key, f);
    }

    pub async fn start_async(&mut self) {
        while let Some(key) = self.receiver.recv().await {
            let hander = self.handlers.get(&key).unwrap();
            hander();
        }
    }
}

impl Default for Dispatch {
    fn default() -> Self {
        Self::new()
    }
}
