use egui_plot::{Legend, Line, PlotPoints};

use super::repository_link;

pub struct EguiPlot {}

impl EguiPlot {
    pub fn show(ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/bircni/egui-example/tree/master",
                egui::RichText::new("(source code)").small()
            ));
            repository_link(ui, "https://github.com/emilk/egui_plot");
        });
        egui_plot::Plot::new("plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(f64::sin, .., 5000)).name("Sinus"),
                );
                plot_ui.line(
                    Line::new(PlotPoints::from_explicit_callback(f64::cos, .., 5000))
                        .name("Cosinus"),
                );
            });
    }
}
