use ggez::event::KeyCode;
use rapier2d::geometry::ColliderHandle;

#[derive(Debug, strum_macros::ToString, strum_macros::AsRefStr, Clone, Copy)]
pub enum Event {
    KeyPressed(KeyCode),
    IntersectionEvent(ColliderHandle, ColliderHandle),
}
