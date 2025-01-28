use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        thread_local! {
            static LAST_CLICKED_NODE: std::cell::RefCell<Option<NodeId>> = std::cell::RefCell::new(None);
        }

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        let background_color = Color32::LIGHT_GRAY; // Background nero
        painter.rect_filled(response.rect, 0.0, background_color);

        let panel_offset = response.rect.min;
        let pointer_position = ui.input(|i| i.pointer.interact_pos());

        let mut clicked_node_id: Option<NodeId> = None;

        // Disegna le linee
        for elem in &self.nodi {
            for &neighbour in &elem.neighbours {
                if let Some(neighbour_node) = self.nodi.iter().find(|node| node.node_id == neighbour) {
                    let is_connected_to_clicked = LAST_CLICKED_NODE.with(|last_clicked| {
                        let last_clicked = *last_clicked.borrow();
                        last_clicked == Some(elem.node_id) || last_clicked == Some(neighbour_node.node_id)
                    });

                    // Linee collegate al nodo selezionato diventano grigie
                    let line_color = if is_connected_to_clicked {
                        Color32::GRAY
                    } else {
                        Color32::BLACK
                    };

                    painter.line_segment(
                        [
                            Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                            Pos2::new(neighbour_node.xy.0 + panel_offset.x, neighbour_node.xy.1 + panel_offset.y),
                        ],
                        Stroke::new(2.0, line_color),
                    );
                }
            }
        }

        // Determina quale nodo è stato cliccato
        for elem in self.nodi.clone() {
            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);
            if let Some(pointer) = pointer_position {
                let distance = position.distance(pointer);
                if distance <= 30.0 && ui.input(|i| i.pointer.any_click()) {
                    clicked_node_id = Some(elem.node_id);

                    if let SimulationControllerNodeType::CLIENT { .. } = elem.node_type {
                        self.open_client_popup(*elem);
                    }
                }
            }
        }

        // Aggiorna il nodo cliccato
        if ui.input(|i| i.pointer.any_click()) {
            if let Some(node_id) = clicked_node_id {
                LAST_CLICKED_NODE.with(|last_clicked| {
                    *last_clicked.borrow_mut() = Some(node_id);
                });
            } else {
                LAST_CLICKED_NODE.with(|last_clicked| {
                    *last_clicked.borrow_mut() = None;
                });
            }
        }

        // Disegna i nodi
        for elem in &mut self.nodi {
            let rect = egui::Rect::from_center_size(
                Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                egui::vec2(60.0, 60.0),
            );
            let response = ui.allocate_rect(rect, egui::Sense::drag());

            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);

            let fill_color = match elem.node_type {
                SimulationControllerNodeType::SERVER { .. } => Color32::LIGHT_RED,
                SimulationControllerNodeType::CLIENT { .. } => Color32::LIGHT_GREEN,
                SimulationControllerNodeType::DRONE { .. } => Color32::LIGHT_BLUE,
            };

            // Verifica se il nodo è selezionato
            let is_selected = LAST_CLICKED_NODE.with(|last_clicked| *last_clicked.borrow() == Some(elem.node_id));

            // Determina spessore e colore del bordo
            let stroke_thickness = if is_selected { 4.0 } else { 2.0 };
            let stroke_color = Color32::BLACK;

            // Disegna il cerchio
            painter.circle(
                position,
                30.0,
                fill_color,
                Stroke::new(stroke_thickness, stroke_color),
            );

            // Dimensione, stile e colore della label
            let font_size = if is_selected { 24.0 } else { 20.0 };
            let font_weight = if is_selected { egui::FontId::monospace(font_size) } else { egui::FontId::proportional(font_size) };
            let label_color = Color32::BLACK;

            // Label del nodo
            let letter = match elem.node_type {
                SimulationControllerNodeType::SERVER { .. } => "S",
                SimulationControllerNodeType::CLIENT { .. } => "C",
                SimulationControllerNodeType::DRONE { .. } => "D",
            };

            painter.text(
                position,
                egui::Align2::CENTER_CENTER,
                format!("{}{}", letter, elem.node_id),
                font_weight,
                label_color,
            );

            // Aggiorna la posizione del nodo durante il drag
            if response.dragged() {
                let drag_delta = response.drag_delta();
                elem.xy.0 += drag_delta.x;
                elem.xy.1 += drag_delta.y;
            }
        }
    }
}
