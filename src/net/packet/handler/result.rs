pub struct HandlerResult {
    pub model: Vec<Action>,
}

impl HandlerResult {
    pub fn new() -> Self {
        Self { model: Vec::new() }
    }

    pub fn add_action(&mut self, action: T) -> () {
        self.model.push(action);
    }
}
