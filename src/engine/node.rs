#[derive(Debug, Clone)]
pub struct Node {
    inner: web_sys::Node,
}

impl Node {
    pub fn new(inner: web_sys::Node) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &web_sys::Node {
        &self.inner
    }
}
