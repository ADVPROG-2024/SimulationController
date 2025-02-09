use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    pub fn right_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("NODE LIST".to_string());
        for elem in &self.nodi {
            let is_selected = if let Some(selected_node) = &self.panel.central_panel.selected_node {
                selected_node.node_id == elem.node_id
            } else { false };
            // Crea una riga orizzontale per il nome e la freccetta
            ui.horizontal(|ui| {
                ui.add_space(10.);
                // Mostra il nome come una label cliccabile
                match elem.clone().node_type{
                    SimulationControllerNodeType::SERVER { .. } => {
                        if ui.label(format!("Server {}",elem.node_id)).clicked() {
                            // Se l'elemento è già selezionato, deselezionalo
                            if is_selected {
                                self.panel.central_panel.selected_node = None;
                            } else {
                                // Altrimenti, seleziona l'elemento corrente
                                self.panel.central_panel.selected_node = Some(elem.clone());
                            }
                        }
                    }
                    SimulationControllerNodeType::CLIENT { .. } => {
                        if ui.label(format!("Client {}",elem.node_id)).clicked() {
                            // Se l'elemento è già selezionato, deselezionalo
                            if is_selected {
                                self.panel.central_panel.selected_node = None;
                            } else {
                                // Altrimenti, seleziona l'elemento corrente
                                self.panel.central_panel.selected_node = Some(elem.clone());
                            }
                        }
                    }
                    SimulationControllerNodeType::DRONE { .. } => {
                        if ui.label(format!("Drone {}",elem.node_id)).clicked() {
                            // Se l'elemento è già selezionato, deselezionalo
                            if is_selected {
                                self.panel.central_panel.selected_node = None;
                            } else {
                                // Altrimenti, seleziona l'elemento corrente
                                self.panel.central_panel.selected_node = Some(elem.clone());
                            }
                        }
                    }
                }

                // Mostra una freccetta (icona) cliccabile
                let freccetta = if is_selected {
                    "▼" // Freccetta verso il basso (elemento aperto)
                } else {
                    "▶" // Freccetta verso destra (elemento chiuso)
                };

                if ui.button(freccetta).clicked() {
                    // Se l'elemento è già selezionato, deselezionalo
                    if is_selected {
                        self.panel.central_panel.selected_node = None;
                    } else {
                        // Altrimenti, seleziona l'elemento corrente
                        self.panel.central_panel.selected_node = Some(elem.clone());
                    }
                }
            });

            // Se l'elemento è selezionato, mostra i dettagli sotto di esso
            if is_selected {
                ui.label(format!("Vicini: {:?}", elem.neighbours.clone()));
            }
        }
    }
}