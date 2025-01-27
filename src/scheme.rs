use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click());
        let background_color = Color32::GRAY;
        painter.rect_filled(response.rect, 0.0, background_color);

        let panel_offset = response.rect.min;
        let pointer_position = ui.input(|i| i.pointer.interact_pos());

        // Variabile per memorizzare il nodo cliccato
        let mut clicked_node_id: Option<NodeId> = None;

        // Disegna connessioni e nodi
        for elem in &self.nodi {
            for &neighbour in &elem.neighbours {
                if let Some(neighbour_node) = self.nodi.iter().find(|node| node.node_id == neighbour) {
                    painter.line_segment(
                        [
                            Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                            Pos2::new(neighbour_node.xy.0 + panel_offset.x, neighbour_node.xy.1 + panel_offset.y),
                        ],
                        Stroke::new(2.0, Color32::BLACK),
                    );
                }
            }
        }

        for elem in &mut self.nodi {
            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);

            // Determina se questo nodo è cliccato
            if let Some(pointer) = pointer_position {
                let distance = position.distance(pointer);
                if distance <= 30.0 && ui.input(|i| i.pointer.any_click()) {
                    clicked_node_id = Some(elem.node_id);
                }
            }

            // Determina il colore del nodo
            let fill_color = if clicked_node_id == Some(elem.node_id) {
                Color32::YELLOW // Nodo cliccato evidenziato
            } else {
                match elem.node_type {
                    SimulationControllerNodeType::SERVER { .. } => Color32::LIGHT_RED,
                    SimulationControllerNodeType::CLIENT { .. } => Color32::LIGHT_GREEN,
                    SimulationControllerNodeType::DRONE { .. } => Color32::LIGHT_BLUE,
                }
            };

            painter.circle(
                position,
                30.0,
                fill_color,
                Stroke::new(1.0, Color32::BLACK),
            );

            let letter = match elem.node_type {
                SimulationControllerNodeType::SERVER { .. } => "S",
                SimulationControllerNodeType::CLIENT { .. } => "C",
                SimulationControllerNodeType::DRONE { .. } => "D",
            };

            painter.text(
                position,
                egui::Align2::CENTER_CENTER,
                format!("{}{}", letter, elem.node_id),
                egui::FontId::proportional(20.0),
                Color32::BLACK,
            );
        }

        // Stampa o usa il nodo cliccato
        if let Some(node_id) = clicked_node_id {
            println!("Nodo cliccato: {}", node_id);
        }
    }
}
