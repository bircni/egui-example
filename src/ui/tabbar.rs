use egui::{global_dark_light_mode_buttons, Align, Layout, Ui};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::App;

impl App {
    pub fn show_tabbar(&mut self, ui: &mut Ui) {
        let selected_tab = &mut self.tab;
        ui.horizontal(|ui| {
            for tab in Tab::iter() {
                ui.selectable_value(selected_tab, tab, tab.to_string());
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                global_dark_light_mode_buttons(ui);
            });
        });
    }
}

#[derive(Copy, Clone, PartialEq, Default, EnumIter, Display)]
pub enum Tab {
    #[default]
    Intro,
    EguiNotify,
}
