use anyhow::Context;
use egui::ViewportBuilder;

mod ui;

fn main() -> anyhow::Result<()> {
    let viewport = ViewportBuilder::default()
        .with_title("egui-example")
        .with_inner_size(egui::vec2(900.0, 600.0));

    eframe::run_native(
        "egui-example",
        eframe::NativeOptions {
            viewport,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(ui::App::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))
    .context("Failed to run native")
}
