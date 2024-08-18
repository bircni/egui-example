use std::path::PathBuf;

use egui_file_dialog::FileDialog;

use super::repository_link;

pub struct EguiFileDialog {
    file_dialog: FileDialog,
    selected_path: Option<PathBuf>,
}

impl EguiFileDialog {
    pub fn new() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            selected_path: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/bircni/egui-example/tree/master",
                egui::RichText::new("(source code)").small()
            ));
            repository_link(ui, "https://github.com/fluxxcode/egui-file-dialog");
        });

        if let Some(path) = self.file_dialog.take_selected() {
            self.selected_path = Some(path);
        }
        ui.label(format!(
            "Selected file: {}",
            self.selected_path
                .as_ref()
                .map_or_else(|| "None".to_owned(), |p| p.to_string_lossy().to_string())
        ));
        ui.button("Select file").clicked().then(|| {
            self.file_dialog.select_file();
        });
        ui.button("Select directory")
            .clicked()
            .then(|| self.file_dialog.select_directory());

        self.file_dialog.update(ui.ctx());
    }
}
