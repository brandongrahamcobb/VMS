use action::model::Action;

#[derive(Clone)]
pub struct HandlerResult {
    pub model: Vec<Action>,
}

impl Default for HandlerResult {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlerResult {
    pub fn new() -> Self {
        Self { model: Vec::new() }
    }

    pub fn add_action(&mut self, action: Action) {
        self.model.push(action);
    }
}
