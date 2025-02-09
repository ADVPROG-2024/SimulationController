use std::fmt::Debug;
use dronegowski_utils::hosts::{ClientEvent, ServerEvent};
use eframe::egui;
use wg_2024::controller::DroneEvent;
use wg_2024::packet::PacketType;
use crate::DronegowskiSimulationController;
use crate::sc_utils::Event;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        for elem in &self.panel.bottom_left_panel.event{
            let mut node_id;
            match elem {
                Event::DroneEvent(drone_event) => {
                    match drone_event {
                        DroneEvent::PacketSent (packet) => {
                            node_id = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Drone {} received a {:?}", node_id, packet.pack_type));
                        }
                        DroneEvent::PacketDropped(packet) => {
                            node_id = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Drone {} dropped a {:?}", node_id, packet.pack_type));
                        }
                        DroneEvent::ControllerShortcut(packet) => {
                            node_id = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Drone {} directed send to Simulation Controller a {:?}", node_id, packet.pack_type));
                        }
                    }


                }
                Event::ClientEvent (client_event) => {
                    match client_event {
                        ClientEvent::PacketSent (packet) => {
                            node_id = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Client {} received a {:?}", node_id, packet.pack_type));
                        }
                        ClientEvent::MessageReceived(_) => {}
                        ClientEvent::ServerTypeReceived(_, _, _) => {}
                        ClientEvent::ClientListReceived(_, _, _) => {}
                        ClientEvent::FilesListReceived(_, _, _) => {}
                        ClientEvent::FileReceived(_, _, _) => {}
                        ClientEvent::MediaReceived(_, _, _) => {}
                        ClientEvent::MessageFromReceived(_, _, _, _) => {}
                        ClientEvent::RegistrationOk(_, _) => {}
                        ClientEvent::RegistrationError(_, _) => {}
                    }
                }
                Event::ServerEvent (server_event) => {
                    match server_event {
                        ServerEvent::PacketSent (packet) => {
                            node_id = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Server {} received a {:?}", node_id, packet.pack_type));
                        }
                        ServerEvent::MessageReceived(_) => {}
                    }
                }
            }
        }
    }
}
