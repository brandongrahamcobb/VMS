use crate::model::Action;

#[derive(Clone)]
pub struct TickEvent {
    pub model: Vec<Action>,
}

impl TickEvent {
    pub fn new() -> Self {
        Self { model: Vec::new() }
    }

    pub fn add_action(&mut self, action: Action) {
        self.model.push(action);
    }
}
