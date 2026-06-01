use action::model::Action;
use bevy::prelude::Message;

#[derive(Message)]
pub struct HandlerResult {
    pub client_id: i32,
    pub actions: Vec<Action>,
}
