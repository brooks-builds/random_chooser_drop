use crossbeam::channel::Sender;
use rapier2d::pipeline::EventHandler;

use crate::event_manager::event::Event;
use crate::event_manager::EventManager;

pub struct PhysicsEventHandler {
    event_sender: Sender<Event>,
}

impl PhysicsEventHandler {
    pub fn new(event_manager: &mut EventManager) -> Self {
        Self {
            event_sender: event_manager.get_sender(),
        }
    }
}

impl EventHandler for PhysicsEventHandler {
    fn handle_intersection_event(&self, event: rapier2d::geometry::IntersectionEvent) {
        if event.intersecting {
            self.event_sender
                .send(Event::IntersectionEvent(event.collider1, event.collider2))
                .unwrap();
        }
    }

    fn handle_contact_event(&self, _event: rapier2d::geometry::ContactEvent) {}
}
