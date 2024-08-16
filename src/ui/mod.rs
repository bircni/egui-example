use eframe::CreationContext;
use egui::{vec2, CentralPanel, Context, TextStyle};
use examples::{egui_notify::EguiNotify, egui_phosphor::EguiPhosphor};
use tabbar::Tab;

mod examples;
mod tabbar;

pub struct App {
    tab: Tab,
    egui_notify: EguiNotify,
    egui_phosphor: EguiPhosphor,
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
        Self {
            tab: Tab::default(),
            egui_notify: EguiNotify::new(),
            egui_phosphor: EguiPhosphor::new(cc),
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
                    self.egui_phosphor.show(ui);
                }
            }
        });
    }
}
