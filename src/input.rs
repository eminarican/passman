use crossterm::event::KeyCode;
use std::sync::mpsc::{
    Receiver,
    channel
};
use std::time::Duration;

pub struct Input {
    queue: Receiver<Event>
}

pub enum Event {
    Input(KeyCode),
    Tick
}

pub fn new() -> Input {
    let (sc, rc) = channel();
    let input = Input{
        queue: rc
    };
    let queue = sc.clone();
    std::thread::spawn(move || {
        loop {
            let event = crossterm::event::read();
            if let None = event.as_ref().err() {
                if let crossterm::event::Event::Key(key) = event.unwrap() {
                    let _ = queue.send(Event::Input(key.code));
                }
            }
        }
    });
    std::thread::spawn(move || loop {
        if let Err(err) = sc.send(Event::Tick) {
            eprintln!("{}", err);
            break;
        }
        std::thread::sleep(Duration::from_millis(250));
    });
    return input
}

impl Input {
    pub fn handle(&self) -> Option<KeyCode> {
        let result = self.queue.recv();
        return if let None = result.as_ref().err() {
            if let Event::Input(key) = result.unwrap() {
                return Some(key)
            }
            None
        } else {
            None
        }
    }
}
