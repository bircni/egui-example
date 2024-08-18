use egui::{Ui, WidgetText};
use egui_tiles::{TileId, UiResponse};

use super::repository_link;

struct Pane {
    nr: usize,
}

pub struct EguiTiles {}

impl EguiTiles {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/bircni/egui-example/tree/master",
                egui::RichText::new("(source code)").small()
            ));
            repository_link(ui, "https://github.com/rerun-io/egui_tiles");
        });

        let mut tree = create_tree();
        tree.ui(self, ui);
    }
}

impl egui_tiles::Behavior<Pane> for EguiTiles {
    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        // Give each pane a unique color:
        let color = egui::epaint::Hsva::new(0.103 * pane.nr as f32, 0.5, 0.5, 1.0);
        ui.painter().rect_filled(ui.max_rect(), 0.0, color);

        ui.label(format!("The contents of pane {}.", pane.nr));

        // You can make your pane draggable like so:
        if ui
            .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
            .drag_started()
        {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        format!("Pane {}", pane.nr).into()
    }
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut next_view_nr = 0;
    let mut gen_pane = || {
        let pane = Pane { nr: next_view_nr };
        next_view_nr += 1;
        pane
    };

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];
    tabs.push({
        let children = (0..7).map(|_| tiles.insert_pane(gen_pane())).collect();
        tiles.insert_horizontal_tile(children)
    });

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new("my_tree", root, tiles)
}
