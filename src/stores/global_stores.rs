pub trait GlobalStoreTrait {
    fn set_initialize_app(&mut self, initialized: bool);
}

pub struct GlobalStore {
    initializing_app: bool,
}

impl GlobalStore {
    pub fn default() -> Self {
        Self {
            initializing_app: true,
        }
    }
}

impl GlobalStoreTrait for GlobalStore {
    fn set_initialize_app(&mut self, initialized: bool) {
        self.initializing_app = initialized;
    }
}
