use egui::{Hyperlink, RichText, Ui};

pub mod egui_notify;
pub mod egui_phosphor;
pub mod egui_plot;

pub fn repository_link(ui: &mut Ui, source_code_url: &str) {
    ui.add(Hyperlink::from_label_and_url(
        RichText::new("(repository)").small(),
        source_code_url,
    ));
}
