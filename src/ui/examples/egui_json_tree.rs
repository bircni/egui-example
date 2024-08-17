use egui::Ui;
use egui_json_tree::JsonTree;

use super::repository_link;

pub fn show_egui_json(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.add(egui::github_link_file!(
            "https://github.com/bircni/egui-example/tree/master",
            egui::RichText::new("(source code)").small()
        ));
        repository_link(ui, "https://github.com/dmackdev/egui_json_tree");
    });

    let value =
        serde_json::json!({ "egui": "gui", "rust": ["1.78", "1.79", "1.80"], "cargo": true });

    JsonTree::new("json_tree", &value)
        .abbreviate_root(true)
        .show(ui);
}
