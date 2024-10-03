use eframe::egui::Slider;
use egui::{FontId, Ui};
use egui_notify::{Toast, Toasts};
use std::time::Duration;

use super::repository_link;

pub struct EguiNotify {
    toasts: Toasts,
    caption: String,
    closable: bool,
    show_progress_bar: bool,
    expires: bool,
    duration: f32,
    font_size: f32,
    custom_level_string: String,
    custom_level_color: egui::Color32,
}

impl EguiNotify {
    pub fn new() -> Self {
        Self {
            caption: r"Hello! It's a multiline caption
Next line
Another one
And another one"
                .into(),
            toasts: Toasts::default(),
            closable: true,
            expires: true,
            show_progress_bar: true,
            duration: 3.5,
            font_size: 16.,
            custom_level_string: "$".into(),
            custom_level_color: egui::Color32::GREEN,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/bircni/egui-example/tree/master",
                egui::RichText::new("(source code)").small()
            ));
            repository_link(ui, "https://github.com/ItsEthra/egui-notify");
        });

        ui.text_edit_multiline(&mut self.caption);
        ui.checkbox(&mut self.expires, "Expires");
        ui.checkbox(&mut self.closable, "Closable");
        ui.checkbox(&mut self.show_progress_bar, "ShowProgressBar");
        if !(self.expires || self.closable) {
            ui.label("Warning; toasts will have to be closed programatically");
        }
        ui.add_enabled_ui(self.expires, |ui| {
            ui.horizontal(|ui| {
                ui.label("Duration (in s)");
                ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Font size");
                ui.add(Slider::new(&mut self.font_size, 8.0..=20.0));
            });
        });
        ui.text_edit_singleline(&mut self.custom_level_string);
        ui.color_edit_button_srgba(&mut self.custom_level_color);

        let customize_toast = |t: &mut Toast| {
            let duration = if self.expires {
                Some(Duration::from_millis((1000. * self.duration) as u64))
            } else {
                None
            };
            t.closable(self.closable)
                .duration(duration)
                .show_progress_bar(self.show_progress_bar)
                .font(FontId::proportional(self.font_size));
        };

        ui.horizontal(|ui| {
            if ui.button("Success").clicked() {
                customize_toast(self.toasts.success(self.caption.clone()));
            }

            if ui.button("Info").clicked() {
                customize_toast(self.toasts.info(self.caption.clone()));
            }

            if ui.button("Warning").clicked() {
                customize_toast(self.toasts.warning(self.caption.clone()));
            }

            if ui.button("Error").clicked() {
                customize_toast(self.toasts.error(self.caption.clone()));
            }

            if ui.button("Basic").clicked() {
                customize_toast(self.toasts.basic(self.caption.clone()));
            }

            if ui.button("Custom").clicked() {
                customize_toast(self.toasts.custom(
                    self.caption.clone(),
                    self.custom_level_string.clone(),
                    self.custom_level_color,
                ));
            }

            if ui
                .button("Phosphor")
                .on_hover_text("This toast uses egui-phosphor icons")
                .clicked()
            {
                customize_toast(self.toasts.custom(
                    self.caption.clone(),
                    egui_phosphor::regular::FAN.to_owned(),
                    self.custom_level_color,
                ));
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Dismiss all toasts").clicked() {
                self.toasts.dismiss_all_toasts();
            }
            if ui.button("Dismiss latest toast").clicked() {
                self.toasts.dismiss_latest_toast();
            }
            if ui.button("Dismiss oldest toast").clicked() {
                self.toasts.dismiss_oldest_toast();
            }
        });

        self.toasts.show(ui.ctx());
    }
}
