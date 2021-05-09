use std::collections::HashMap;

use crossbeam::channel::{Receiver, Sender};
use eyre::Result;

use self::event::Event;

pub mod event;

#[derive(Debug)]
pub struct EventManager {
    events_receiver: Receiver<Event>,
    events_sender: Sender<Event>,
    subscribers: HashMap<String, Vec<Sender<Event>>>,
}

impl EventManager {
    pub fn new() -> Self {
        let (events_sender, events_receiver) = crossbeam::channel::unbounded();
        Self {
            events_receiver,
            events_sender,
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_name: String) -> Receiver<Event> {
        let (sender, receiver) = crossbeam::channel::unbounded();
        let subscribers = self.subscribers.entry(event_name).or_default();
        subscribers.push(sender);
        receiver
    }

    pub fn get_sender(&self) -> Sender<Event> {
        self.events_sender.clone()
    }

    pub fn update(&mut self) -> Result<()> {
        loop {
            let event = if let Ok(event) = self.events_receiver.try_recv() {
                event
            } else {
                return Ok(());
            };

            if let Some(subscribers) = self.subscribers.get(event.as_ref()) {
                for subscriber in subscribers {
                    subscriber.send(event)?;
                }
                return Ok(());
            }
        }
    }
}
