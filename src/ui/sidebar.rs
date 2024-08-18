use egui::{global_dark_light_mode_buttons, Context, SidePanel};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::App;

impl App {
    pub fn show_sidebar(&mut self, ctx: &Context) {
        SidePanel::left("settings").show(ctx, |ui| {
            let selected_tab = &mut self.tab;
            ui.vertical(|ui| {
                ui.add_space(6.0);
                global_dark_light_mode_buttons(ui);
                ui.separator();
                for tab in Tab::iter() {
                    ui.selectable_value(selected_tab, tab, tab.to_string());
                }
            });
        });
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, EnumIter, Display)]
pub enum Tab {
    #[default]
    Intro,
    EguiNotify,
    EguiPhosphor,
    EguiPlot,
    EguiForm,
    EguiJsonTree,
    EguiTiles,
    EguiFileDialog,
}
