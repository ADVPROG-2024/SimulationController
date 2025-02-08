use std::fmt::Debug;
use dronegowski_utils::hosts::ClientEvent;
use eframe::egui;
use wg_2024::controller::DroneEvent;
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        for elem in &self.panel.bottom_left_panel.drone_event{
            match elem {
                DroneEvent::PacketSent(..) =>{
                    ui.label("Drone received packet sent");
                }
                DroneEvent::PacketDropped(_) => {
                    ui.label("Drone received packet dropped");

                }
                DroneEvent::ControllerShortcut(_) => {
                    ui.label("Drone received controller shortcut");

                }
            }
        }

        for elem in &self.panel.bottom_left_panel.client_event{
            match elem {
                ClientEvent::ServerTypeReceived(client_id, server_id, server_type) => {
                    ui.label(format!("Client {} received 'Im {:?}' from Server {:?}", self.panel.central_panel.selected_node.clone().unwrap().node_id, server_type, server_id));
                }
                ClientEvent::ClientListReceived(client_id, server_id, clients) => {}
                ClientEvent::FilesListReceived(client_id, server_id, files) => {}
                ClientEvent::FileReceived(client_id, server_id, file_data) => {}
                ClientEvent::MediaReceived(client_id, server_id, media_data) => {}
                ClientEvent::MessageFromReceived(client_id, server_id, from_id, message) => {}
                ClientEvent::RegistrationOk(client_id, server_id) => {}
                ClientEvent::RegistrationError(client_id, server_id) => {}

                ClientEvent::PacketSent(_) => {}
                ClientEvent::MessageReceived(_) => {}
            }
        }

        for elem in &self.panel.bottom_left_panel.server_event{
            //ui.label(format!("{:?}", elem));
        }
    }
}
