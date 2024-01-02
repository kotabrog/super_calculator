#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    inner: web_sys::KeyboardEvent,
}

impl KeyboardEvent {
    pub fn new(inner: web_sys::KeyboardEvent) -> Self {
        Self { inner }
    }

    pub fn key(&self) -> String {
        self.inner.key()
    }

    pub fn ctrl_key(&self) -> bool {
        self.inner.ctrl_key()
    }

    pub fn ctrl_enter(&self) -> bool {
        self.ctrl_key() && self.key() == "Enter"
    }
}
