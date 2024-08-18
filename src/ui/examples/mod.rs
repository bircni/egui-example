use egui::{Hyperlink, RichText, Ui};

pub mod egui_file;
pub mod egui_file_dialog;
pub mod egui_form;
pub mod egui_json_tree;
pub mod egui_notify;
pub mod egui_phosphor;
pub mod egui_plot;
pub mod egui_tiles;

pub fn repository_link(ui: &mut Ui, source_code_url: &str) {
    ui.add(Hyperlink::from_label_and_url(
        RichText::new("(repository)").small(),
        source_code_url,
    ));
}
