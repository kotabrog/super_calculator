// use anyhow::{anyhow, Result};
// use wasm_bindgen::JsCast;
// use web_sys::{Node, HtmlElement};

// pub fn get_node_from_html_element(element: HtmlElement) -> Result<Node> {
//     element
//         .dyn_into::<Node>()
//         .map_err(|element| anyhow!("Error converting {:#?} to Node", element))
// }
