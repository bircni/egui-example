use egui::ScrollArea;
use egui_phosphor::{bold, fill, light, regular, thin};

use super::repository_link;

pub fn init_egui_phosphor(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();

    // Add the phosphor regular font to the font definitions separately for egui-notify
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    fonts.font_data.insert(
        "phosphor-thin".into(),
        egui_phosphor::Variant::Thin.font_data().into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("phosphor-thin".into()),
        vec!["Ubuntu-Light".into(), "phosphor-thin".into()],
    );

    fonts.font_data.insert(
        "phosphor-light".into(),
        egui_phosphor::Variant::Light.font_data().into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("phosphor-light".into()),
        vec!["Ubuntu-Light".into(), "phosphor-light".into()],
    );

    fonts.font_data.insert(
        "phosphor".into(),
        egui_phosphor::Variant::Regular.font_data().into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("phosphor".into()),
        vec!["Ubuntu-Light".into(), "phosphor".into()],
    );

    fonts.font_data.insert(
        "phosphor-bold".into(),
        egui_phosphor::Variant::Bold.font_data().into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("phosphor-bold".into()),
        vec!["Ubuntu-Light".into(), "phosphor-bold".into()],
    );

    fonts.font_data.insert(
        "phosphor-fill".into(),
        egui_phosphor::Variant::Fill.font_data().into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("phosphor-fill".into()),
        vec!["Ubuntu-Light".into(), "phosphor-fill".into()],
    );

    cc.egui_ctx.set_fonts(fonts);
}

pub fn show_egui_phosphor(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add(egui::github_link_file!(
            "https://github.com/bircni/egui-example/tree/master",
            egui::RichText::new("(source code)").small()
        ));
        repository_link(ui, "https://github.com/amPerl/egui-phosphor");
    });

    ScrollArea::vertical()
        .max_width(f32::INFINITY)
        .show(ui, |ui| {
            let fonts = [
                ("phosphor-thin", thin::FILE_CODE),
                ("phosphor-light", light::FILE_CODE),
                ("phosphor", regular::FILE_CODE),
                ("phosphor-bold", bold::FILE_CODE),
                ("phosphor-fill", fill::FILE_CODE),
            ];

            let mut fonts_iter = fonts.iter();

            while let Some((family1, icon1)) = fonts_iter.next() {
                ui.horizontal(|ui| {
                    render_font_family(ui, family1, icon1);

                    if let Some((family2, icon2)) = fonts_iter.next() {
                        render_font_family(ui, family2, icon2);
                    }
                });
            }
        });
}

fn render_font_family(ui: &mut egui::Ui, family: &str, icon: &str) {
    ui.vertical(|ui| {
        ui.heading(family);
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            for size in [16.0, 32.0, 48.0] {
                let demo_text = format!("FILE_CODE {icon}");
                ui.label(
                    egui::RichText::new(&demo_text)
                        .family(egui::FontFamily::Name(family.into()))
                        .size(size),
                );
            }
        });
    });
}
