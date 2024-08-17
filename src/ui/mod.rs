use eframe::CreationContext;
use egui::{vec2, CentralPanel, Context, TextStyle};
use examples::{
    egui_form::EguiForm, egui_json_tree, egui_notify::EguiNotify, egui_phosphor, egui_plot,
    egui_tiles::EguiTiles,
};
use tabbar::Tab;

mod examples;
mod tabbar;

pub struct App {
    tab: Tab,
    egui_notify: EguiNotify,
    egui_form: EguiForm,
    egui_tiles: EguiTiles,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.style_mut(|s| {
            s.text_styles.insert(
                TextStyle::Name("subheading".into()),
                TextStyle::Monospace.resolve(s),
            );
            s.text_styles
                .insert(TextStyle::Body, TextStyle::Monospace.resolve(s));
            s.spacing.item_spacing = vec2(10.0, std::f32::consts::PI * 1.76643);
        });
        egui_phosphor::init_egui_phosphor(cc);
        Self {
            tab: Tab::default(),
            egui_notify: EguiNotify::new(),
            egui_form: EguiForm::new(),
            egui_tiles: EguiTiles {},
        }
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_tabbar(ui);

            ui.vertical_centered(|ui| {
                ui.separator();
            });

            match self.tab {
                Tab::Intro => {
                    ui.label("Welcome to egui-example!");
                    ui.label("Have fun exploring the examples.");
                    ui.label(
                        "Also feel free to check out the source code or add your own examples.",
                    );
                    ui.label("Select a tab to get started.");
                }
                Tab::EguiNotify => {
                    self.egui_notify.show(ui);
                }
                Tab::EguiPhosphor => {
                    egui_phosphor::show_egui_phosphor(ui);
                }
                Tab::EguiPlot => {
                    egui_plot::show_egui_plot(ui);
                }
                Tab::EguiForm => {
                    self.egui_form.show(ui);
                }
                Tab::EguiJsonTree => {
                    egui_json_tree::show_egui_json(ui);
                }
                Tab::EguiTiles => {
                    self.egui_tiles.show(ui);
                }
            }
        });
    }
}
