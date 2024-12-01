use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use crate::{MyEguiApp, NodeType};

impl MyEguiApp {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(egui::Vec2::splat(580.0), egui::Sense::drag());
        let background_color = Color32::GRAY;
        painter.rect_filled(
            response.rect,
            0.0,
            background_color,
        );

        for (id1, id2) in &self.neighbours{
            let xy1 = self.nodi.iter().find(|node| node.node_id == *id1).unwrap().xy;
            let xy2 = self.nodi.iter().find(|node| node.node_id == *id2).unwrap().xy;

            painter.line_segment(
                [Pos2::new(xy1.0,xy1.1), Pos2::new(xy2.0, xy2.1)],
                Stroke::new(2.0, Color32::BLACK),
            );
        }
        for elem in &mut self.nodi{
            let fill_color = match elem.node_type {
                NodeType::SERVER => Color32::LIGHT_RED,
                NodeType::CLIENT => Color32::LIGHT_GREEN,
                NodeType::DRONE => Color32::LIGHT_BLUE,
            };

            let rect = egui::Rect::from_center_size(Pos2::new(elem.xy.0, elem.xy.1), egui::vec2(60.0, 60.0));
            let response = ui.allocate_rect(rect, egui::Sense::drag());
            painter.circle(
                Pos2::new(elem.xy.0, elem.xy.1),
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
                Pos2::new(elem.xy.0, elem.xy.1),
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
