use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use egui_file::FileDialog;

use super::repository_link;

#[derive(Default)]
pub struct EguiFile {
    open_file: Option<PathBuf>,
    file_dialog: Option<FileDialog>,
}

impl EguiFile {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/bircni/egui-example/tree/master",
                egui::RichText::new("(source code)").small()
            ));
            repository_link(ui, "https://github.com/Barugon/egui_file");
        });

        if let Some(dialog) = &mut self.file_dialog {
            if dialog.show(ui.ctx()).selected() {
                if let Some(file) = dialog.path() {
                    self.open_file = Some(file.to_path_buf());
                }
            }
        }

        ui.label(format!(
            "Selected file: {}",
            self.open_file
                .as_ref()
                .map_or_else(|| "None".to_owned(), |p| p.to_string_lossy().to_string())
        ));
        if (ui.button("Open text file")).clicked() {
            let filter = Box::new({
                let ext = Some(OsStr::new("txt"));
                move |path: &Path| -> bool { path.extension() == ext }
            });
            let mut dialog =
                FileDialog::open_file(self.open_file.clone()).show_files_filter(filter);
            dialog.open();
            self.file_dialog = Some(dialog);
        }

        if (ui.button("Open any file")).clicked() {
            let mut dialog = FileDialog::open_file(self.open_file.clone());
            dialog.open();
            self.file_dialog = Some(dialog);
        }
    }
}
