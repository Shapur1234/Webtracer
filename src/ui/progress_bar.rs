use crate::misc::get_element_by_id;
use web_sys::HtmlElement;

// --------------------------------------------------

#[derive(Clone, Debug)]
pub struct ProgressBar {
    text_element: HtmlElement,
}

#[allow(dead_code)]
impl ProgressBar {
    pub fn set_percentage(&self, percent: f32) {
        self.text_element
            .set_text_content(Some(format!("{percent:.3} %").as_str()));
    }
}

impl Default for ProgressBar {
    fn default() -> ProgressBar {
        let text_element = get_element_by_id("progress_text");
        text_element
            .style()
            .set_property("display", "block")
            .unwrap();
        text_element.set_text_content(Some("0.000 %"));

        ProgressBar { text_element }
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        self.text_element.set_text_content(None);
        self.text_element
            .style()
            .set_property("display", "none")
            .unwrap();
    }
}
