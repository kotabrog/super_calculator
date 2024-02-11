#[derive(Debug, Clone)]
pub struct HtmlSelectElement {
    inner: web_sys::HtmlSelectElement,
}

impl HtmlSelectElement {
    pub fn new(inner: web_sys::HtmlSelectElement) -> Self {
        Self { inner }
    }

    // pub fn new_from_id(id: &str) -> Result<Self> {
    //     Ok(Self::new(get_html_select_element_by_id(id)?))
    // }

    pub fn get_value(&self) -> String {
        self.inner.value()
    }
}
