use crossterm::event::KeyCode;
use std::sync::mpsc::{
    SyncSender,
    Receiver,
    channel
};

pub struct Input {
    queue: Receiver<KeyCode>
}

pub fn new() -> Input {
    let (sc, rc) = channel();
    let input = Input{
        queue: rc
    };
    std::thread::spawn(move || {
        let queue = sc.clone();
        loop {
            let event = crossterm::event::read();
            if let None = event.as_ref().err() {
                if let crossterm::event::Event::Key(key) = event.unwrap() {
                    let _ = queue.send(key.code);
                }
            }
        }
    });
    return input
}

impl Input {
    pub fn handle(&self) -> Option<KeyCode> {
        let result = self.queue.recv();
        return if let None = result.err() {
            Some(result.unwrap())
        } else {
            None
        }
    }
}
