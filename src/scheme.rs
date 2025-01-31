use std::ops::Deref;
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui;
use eframe::egui::{Color32, Painter, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.ctx().screen_rect().size(), egui::Sense::click_and_drag());
        let background_color = Color32::LIGHT_GRAY;
        painter.rect_filled(response.rect, 0.0, background_color);

        let panel_offset = response.rect.min;
        let pointer_position = ui.input(|i| i.pointer.interact_pos());
        let hover_position = ui.input(|i| i.pointer.hover_pos());


        let mut hovered_node_id: Option<&SimulationControllerNode> = None;
        let mut clicked_node_id: Option<SimulationControllerNode> = None;

        // Disegna le linee
        for elem in &self.nodi {
            for &neighbour in &elem.neighbours {
                if let Some(neighbour_node) = self.nodi.iter().find(|node| node.node_id == neighbour) {
                    let is_connected_to_clicked = if let Some(selected_node) = &self.panel.central_panel.selected_node {
                        selected_node.node_id == elem.node_id || selected_node.node_id == neighbour_node.node_id
                    } else {
                        false // Se non c'è nessun nodo selezionato, non consideriamo una connessione
                    };

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
                        Stroke::new(4.0, line_color),
                    );
                }
            }
        }


        // Determina quale nodo è stato cliccato
        for elem in &self.nodi.clone() {
            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);

            if let Some(pointer) = hover_position {
                let distance = position.distance(pointer);

                if self.panel.bottom_panel.active_add_sender && distance <= 50.0 {
                    hovered_node_id = Some(elem);
                    let start_pos = Pos2::new(&self.panel.central_panel.selected_node.clone().unwrap().xy.0 + panel_offset.x, &self.panel.central_panel.selected_node.clone().unwrap().xy.1 + panel_offset.y);

                    if self.panel.central_panel.selected_node.clone().unwrap().neighbours.contains(&elem.node_id.clone()) {
                        draw_dashed_line(&painter, start_pos, pointer, Stroke::new(4.0, Color32::RED), 10.0, 5.0);
                    } else {
                        draw_dashed_line(&painter, start_pos, pointer, Stroke::new(4.0, Color32::GREEN), 10.0, 5.0);
                        if ui.input(|i| i.pointer.any_click()) {
                            self.add_sender(elem.node_id);
                        }
                    }

                } else if distance <= 50.0 && ui.input(|i| i.pointer.any_click()) && !self.panel.bottom_panel.active_add_sender {
                    clicked_node_id = Some(elem.clone());

                    self.panel.central_panel.selected_node = clicked_node_id.clone();

                    if let SimulationControllerNodeType::CLIENT { .. } = elem.node_type {
                        self.open_client_popup(elem);
                    }
                } else if distance > 50. && self.panel.bottom_panel.active_add_sender {
                    if let Some(selected_node) = &self.panel.central_panel.selected_node {
                        if let Some(pointer) = pointer_position {
                            let start_pos = Pos2::new(selected_node.xy.0 + panel_offset.x, selected_node.xy.1 + panel_offset.y);
                            draw_dashed_line(&painter, start_pos, pointer, Stroke::new(4.0, Color32::GRAY), 10.0, 5.0);
                        }
                    }
                }
            }
        }

        let central_panel_rect = response.rect; // Rettangolo che rappresenta l'area del central panel

        if ui.input(|i| i.pointer.any_click()) {
            if let Some(pointer) = pointer_position {
                if central_panel_rect.contains(pointer) {
                    self.panel.reset();
                    if clicked_node_id.is_none(){
                        self.panel.central_panel.selected_node = None;
                    }

                }
            }
        }

        // Disegna i nodi
        for elem in &mut self.nodi {
            let rect = egui::Rect::from_center_size(
                Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                egui::vec2(100., 100.),
            );
            let response = ui.allocate_rect(rect, egui::Sense::drag());

            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);

            let fill_color = match elem.node_type {
                SimulationControllerNodeType::SERVER { .. } => Color32::LIGHT_RED,
                SimulationControllerNodeType::CLIENT { .. } => Color32::LIGHT_GREEN,
                SimulationControllerNodeType::DRONE { .. } => Color32::LIGHT_BLUE,
            };

            // Verifica se il nodo è selezionato
            let is_selected = if let Some(selected_node) = &self.panel.central_panel.selected_node {
                selected_node.node_id == elem.node_id} else { false };

            // Determina spessore e colore del bordo
            let stroke_thickness = if is_selected { 6.0 } else { 4.0 };
            let stroke_color = Color32::BLACK;

            // Disegna il cerchio
            painter.circle(
                position,
                50.0,
                fill_color,
                Stroke::new(stroke_thickness, stroke_color),
            );

            // Dimensione, stile e colore della label
            let font_size = if is_selected { 45.0 } else { 35.0 };
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

fn draw_dashed_line(painter: &Painter, start: Pos2, end: Pos2, stroke: Stroke, dash_length: f32, gap_length: f32) {
    let direction = (end - start).normalized();
    let total_length = start.distance(end);
    let mut current_length = 0.0;

    while current_length < total_length {
        let segment_start = start + direction * current_length;
        let segment_end = start + direction * (current_length + dash_length).min(total_length);

        painter.line_segment([segment_start, segment_end], stroke);
        current_length += dash_length + gap_length;
    }
}