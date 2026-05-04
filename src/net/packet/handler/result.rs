pub struct HandlerResult<T> {
    pub actions: Vec<T>,
}

impl<T> HandlerResult<T> {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: T) -> () {
        self.actions.push(action);
    }
}
