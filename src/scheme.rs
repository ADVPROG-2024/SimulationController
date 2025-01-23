use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use crate::{simulation_controller, NodeType, DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
        let background_color = Color32::GRAY;
        painter.rect_filled(
            response.rect,
            0.0,
            background_color,
        );
        let panel_offset = response.rect.min;

        let rect1 = egui::Rect::from_center_size(Pos2::new(10.0 + panel_offset.x, 10.0 + panel_offset.y), egui::vec2(30.,30.));
        painter.rect_filled(rect1, 0.0, Color32::from_black_alpha(50));
        let response1 = ui.allocate_rect(rect1, egui::Sense::click());

        if response1.clicked(){
            self.left_panel = !self.left_panel;
        }

        for mut elem in self.nodi.clone(){
            for neighbour in elem.neighbours{
                let xy1 = self.nodi.iter().find(|node| node.node_id == neighbour).unwrap().xy;

                painter.line_segment(
                    [
                        Pos2::new(elem.xy.0 + panel_offset.x,elem.xy.1 + panel_offset.y),
                        Pos2::new(xy1.0 + panel_offset.x, xy1.1 + panel_offset.y)
                    ],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }

            let fill_color = match elem.node_type {
                NodeType::SERVER => Color32::LIGHT_RED,
                NodeType::CLIENT => Color32::LIGHT_GREEN,
                NodeType::DRONE => Color32::LIGHT_BLUE,
            };

            let rect = egui::Rect::from_center_size(
                Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                egui::vec2(60.0, 60.0));

            let response = ui.allocate_rect(rect, egui::Sense::drag());

            painter.circle(
                Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                30.0,
                fill_color,   // Fill color
                Stroke::new(1.0, Color32::BLACK),
            );

            let letter = match elem.node_type {
                NodeType::SERVER => "S",
                NodeType::CLIENT => "C",
                NodeType::DRONE => "D",
            };

            painter.text(
                Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                egui::Align2::CENTER_CENTER,
                format!("{}{}",letter, elem.node_id),
                egui::FontId::proportional(20.0),
                Color32::BLACK,
            );

            if response.dragged(){
                let drag_delta = response.drag_delta();
                elem.xy.0 += drag_delta.x;
                elem.xy.1 += drag_delta.y;
            }
        }
    }
}
