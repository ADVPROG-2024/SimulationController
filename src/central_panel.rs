use std::time::{Duration};
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui;
use eframe::egui::{Color32, Painter, Pos2, Stroke};
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController<'_> {
    pub fn central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let (response, painter) = ui.allocate_painter(ui.ctx().screen_rect().size(), egui::Sense::click());
        let background_color = Color32::GRAY;
        painter.rect_filled(response.rect, 0.0, background_color);

        let panel_offset = response.rect.min;
        let pointer_position = ui.input(|i| i.pointer.interact_pos());

        let mut clicked_node_id: Option<SimulationControllerNode> = None;

        for elem in &self.nodi {
            for &neighbour in &elem.neighbours {
                if let Some(neighbour_node) = self.nodi.iter().find(|node| node.node_id == neighbour) {
                    let is_connected_to_clicked = if let Some(selected_node) = &self.panel.central_panel.selected_node {
                        selected_node.node_id == elem.node_id || selected_node.node_id == neighbour_node.node_id
                    } else {
                        false
                    };

                    let line_color = if is_connected_to_clicked {
                        Color32::DARK_GRAY
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


        for elem in &self.nodi.clone() {
            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);

            if let Some(pointer) = pointer_position {
                let distance = position.distance(pointer);

                if self.panel.upper_left_panel.add_sender {
                    self.add_sender_graphic(distance, elem, pointer, panel_offset, &painter, ui);
                }

                else if self.panel.upper_left_panel.remove_sender {
                    self.remove_sender_graphic(distance, elem, ui);
                }

                else if self.panel.upper_left_panel.crash{
                     self.crash_graphic();
                }

                else if distance <= 30.0 && ui.input(|i| i.pointer.any_click()) {
                    clicked_node_id = Some(elem.clone());

                    self.panel.central_panel.selected_node = clicked_node_id.clone();

                    if let SimulationControllerNodeType::CLIENT { .. } = elem.node_type {
                        self.open_client_popup(elem);
                    }
                }
            }
        }

        let central_panel_rect = response.rect;

        if ui.input(|i| i.pointer.any_click()) {
            if let Some(pointer) = pointer_position {
                if central_panel_rect.contains(pointer) {
                    self.panel.reset();
                    if clicked_node_id.is_none() {
                        self.panel.central_panel.selected_node = None;
                    }
                }
            }
        }

        for elem in &mut self.nodi {
            let rect = egui::Rect::from_center_size(
                Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y),
                egui::vec2(60., 60.),
            );
            let response = ui.allocate_rect(rect, egui::Sense::drag());

            let position = Pos2::new(elem.xy.0 + panel_offset.x, elem.xy.1 + panel_offset.y);

            let fill_color = match elem.node_type {
                SimulationControllerNodeType::SERVER { .. } => Color32::LIGHT_RED,
                SimulationControllerNodeType::CLIENT { .. } => Color32::LIGHT_GREEN,
                SimulationControllerNodeType::DRONE { .. } => Color32::LIGHT_BLUE,
            };

            let is_selected = if let Some(selected_node) = &self.panel.central_panel.selected_node {
                selected_node.node_id == elem.node_id
            } else { false };

            let stroke_thickness = if is_selected { 6.0 } else { 4.0 };
            let stroke_color = Color32::BLACK;

            painter.circle(
                position,
                30.0,
                fill_color,
                Stroke::new(stroke_thickness, stroke_color),
            );

            let font_size = if is_selected { 40.0 } else { 30.0 };
            let font_weight = if is_selected { egui::FontId::monospace(font_size) } else { egui::FontId::proportional(font_size) };
            let label_color = Color32::BLACK;

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

            if response.dragged() {
                let drag_delta = response.drag_delta();
                elem.xy.0 += drag_delta.x;
                elem.xy.1 += drag_delta.y;
            }
        }
        if self.panel.central_panel.active_error.is_err(){
            let popup_duration = Duration::from_secs(3);

            if let Some(timer) = self.panel.central_panel.popup_timer{
                if timer.elapsed() >= popup_duration {
                    self.panel.central_panel.active_error = Ok(());
                    self.panel.central_panel.popup_timer = None;
                }
            }

            egui::Window::new("Error")
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .frame(egui::Frame::popup(&ctx.style()).fill(Color32::RED))
                .show(ctx, |ui| {
                    ui.set_max_width(200.0);
                    ui.set_max_height(100.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.colored_label(Color32::WHITE, format!("{:?}", self.panel.central_panel.active_error));
                        ui.add_space(10.0);
                    });
                });
        }
    }

    fn add_sender_graphic(&mut self, distance: f32, elem: &SimulationControllerNode, pointer: Pos2, panel_offset: Pos2, painter: &Painter, ui: &mut egui::Ui) {
        if distance <= 30.0 {

            if self.panel.central_panel.selected_node.clone().unwrap().neighbours.contains(&elem.node_id.clone()) {
            } else {
                if ui.input(|i| i.pointer.any_click()) {
                    self.add_sender(elem.node_id);
                }
            }
        } else if distance > 30. {
            if let Some(selected_node) = &self.panel.central_panel.selected_node {
                let start_pos = Pos2::new(selected_node.xy.0 + panel_offset.x, selected_node.xy.1 + panel_offset.y);
                draw_dashed_line(&painter, start_pos, pointer, Stroke::new(4.0, Color32::DARK_GRAY), 10.0, 5.0);
            }
        }
    }

    fn remove_sender_graphic(&mut self, distance: f32, elem: &SimulationControllerNode, ui: &mut egui::Ui) {
        if distance <= 30. && ui.input(|i| i.pointer.any_click()) {
            if self.panel.central_panel.selected_node.clone().unwrap().neighbours.contains(&elem.node_id.clone()) {
                self.remove_sender(elem.node_id);
            }
        }
    }

    fn crash_graphic(&mut self){
        if let SimulationControllerNodeType::DRONE { .. } = self.panel.central_panel.selected_node.clone().unwrap().node_type {
            self.crash();
        }
    }

    pub fn open_client_popup(&mut self, node: &SimulationControllerNode) {
        self.panel.central_panel.active_popups.insert(node.node_id, node.clone());
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