pub struct HandlerResult<T> {
    pub model: Vec<T>,
}

impl<T> HandlerResult<T> {
    pub fn new() -> Self {
        Self {
            model: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: T) -> () {
        self.model.push(action);
    }
}
