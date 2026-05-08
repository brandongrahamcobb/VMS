use crate::net::action::Action;
use crate::net::error::NetworkError;

pub struct HandlerResult {
    pub model: Vec<Action>,
}

impl HandlerResult {
    pub fn new() -> Self {
        Self { model: Vec::new() }
    }

    pub fn add_action(&mut self, action: Action) -> Result<(), NetworkError> {
        self.model.push(action);
        Ok(())
    }
}
