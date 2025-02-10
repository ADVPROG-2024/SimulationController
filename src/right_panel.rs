use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use eframe::egui::{Align, Button, Direction, Layout, RichText, Rounding, Vec2};
use eframe::epaint::Color32;
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    pub fn right_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(5.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                ui.add_space(5.);
                let close_button = Button::new(RichText::new("X").size(14.0).color(Color32::WHITE)).fill(Color32::RED);
                if ui.add(close_button).clicked() {
                    // Chiudi la finestra
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });

        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new("NODE LIST").size(30.0).color(Color32::BLACK));
            });
        });

        for elem in &mut self.nodi {
            ui.add_space(15.);
            ui.horizontal(|ui| {
                ui.add_space(20.);
                let label_text = match elem.node_type {
                    SimulationControllerNodeType::SERVER { .. } => format!("Server {}", elem.node_id),
                    SimulationControllerNodeType::CLIENT { .. } => format!("Client {}", elem.node_id),
                    SimulationControllerNodeType::DRONE { .. } => format!("Drone {}", elem.node_id),
                };

                let label = RichText::new(label_text).color(Color32::BLACK).size(20.0);
                let response = ui.label(label);

                // Cambia il cursore in una manina se il mouse è sopra la label
                if response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                if response.clicked() {
                    elem.details = !elem.details;
                }

                // Mostra una freccetta (icona) cliccabile
                let freccetta = if elem.details {
                    "v"
                } else {
                    ">"
                };

                let arrow_response = ui.label(RichText::new(freccetta).size(20.).color(Color32::BLACK));

                // Cambia il cursore in una manina se il mouse è sopra la freccetta
                if arrow_response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                if arrow_response.clicked() {
                    elem.details = !elem.details;
                }
            });

            match &elem.node_type {
                SimulationControllerNodeType::DRONE { pdr, .. } => {
                    if elem.details {
                        ui.horizontal(|ui| {
                            ui.add_space(30.);
                            ui.vertical(|ui|{
                                ui.label(RichText::new(format!("PDR: {}", pdr)).size(15.));
                                ui.label(RichText::new(format!("Neighbours: {:?}", elem.neighbours)).size(15.));
                            });
                        });
                    }
                }
                SimulationControllerNodeType::CLIENT { client_type, .. } => {
                    if elem.details {
                        ui.horizontal(|ui| {
                            ui.add_space(20.);
                            ui.vertical(|ui|{
                                ui.label(RichText::new(format!("Client Type: {:?}", client_type)).size(15.));
                                ui.label(RichText::new(format!("Neighbours: {:?}", elem.neighbours)).size(15.));
                            });
                        });
                    }
                }
                SimulationControllerNodeType::SERVER { server_type, .. } => {
                    if elem.details {
                        ui.horizontal(|ui| {
                            ui.add_space(20.);
                            ui.vertical(|ui|{
                                ui.label(RichText::new(format!("Server Type: {:?}", server_type)).size(15.));
                                ui.label(RichText::new(format!("Neighbours: {:?}", elem.neighbours)).size(15.));
                            });
                        });
                    }
                }
            }
        }
    }
}