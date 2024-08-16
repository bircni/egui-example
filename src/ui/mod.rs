use eframe::CreationContext;
use egui::{vec2, CentralPanel, Context, TextStyle};

pub struct App {}

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
        Self {}
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");

            ui.vertical_centered(|ui| {
                ui.separator();
            });

            ui.label("Welcome to egui-example!");
        });
    }
}
