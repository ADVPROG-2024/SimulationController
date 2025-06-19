use std::fmt::Debug;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use dronegowski_utils::network::{Event, SimulationControllerNodeType};
use eframe::egui;
use eframe::egui::{Color32, Direction, Layout, RichText};
use wg_2024::controller::DroneEvent;
use wg_2024::packet::{NackType, PacketType};
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    /// Renders the bottom left panel of the UI, which displays events related to drones, clients, and servers.
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.);

        ui.horizontal(|ui| {
            // Center the "EVENTS" heading horizontally
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new("EVENTS").size(25.0).color(Color32::WHITE));
            });
        });

        ui.add_space(20.);

        // Iterate over the events in the bottom left panel
        for elem in &self.panel.bottom_left_panel.event {
            match elem {
                // Handle drone-related events
                Event::DroneEvent(drone_event) => {
                    match drone_event {
                        // Handle the event when a drone sends a packet
                        DroneEvent::PacketSent(packet) => {
                            match packet.clone().pack_type {
                                // If the packet is a message fragment, notify that a fragment was sent
                                PacketType::MsgFragment(_) => {
                                    self.print_drone_notify(format!("Drone {} sent a fragment to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // If the packet is an ACK, notify that an ACK was sent
                                PacketType::Ack(_) => {
                                    self.print_drone_notify(format!("Drone {} sent an ACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // If the packet is a NACK, handle different types of NACKs
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        // Notify about an ErrorInRouting NACK
                                        NackType::ErrorInRouting(_) => {
                                            self.print_drone_notify(format!("Drone {} sent an ErrorInRouting NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about a DestinationIsDrone NACK
                                        NackType::DestinationIsDrone => {
                                            self.print_drone_notify(format!("Drone {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about a Dropped NACK
                                        NackType::Dropped => {
                                            self.print_drone_notify(format!("Drone {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about an UnexpectedRecipient NACK
                                        NackType::UnexpectedRecipient(_) => {
                                            self.print_drone_notify(format!("Drone {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        // Handle the event when a drone drops a packet
                        DroneEvent::PacketDropped(packet) => {
                            self.print_drone_notify(format!("Drone {} dropped a fragment", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                        }
                        // Handle the event when a drone sends a packet directly to the simulation controller (SC)
                        DroneEvent::ControllerShortcut(packet) => {
                            match packet.pack_type {
                                // Notify about an ACK sent directly to the SC
                                PacketType::Ack(_) => {
                                    self.print_drone_notify(format!("Drone {} sent directly to SC an ACK", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // Notify about a NACK sent directly to the SC
                                PacketType::Nack(_) => {
                                    self.print_drone_notify(format!("Drone {} sent directly to SC a NACK", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // Notify about a FloodResponse sent directly to the SC
                                PacketType::FloodResponse(_) => {
                                    self.print_drone_notify(format!("Drone {} sent directly to SC a FloodResponse", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                _ => {}
                            }

                            // Find the node ID of the destination and send the packet to the appropriate channel
                            let node_id = packet.routing_header.hops[packet.routing_header.hops.len() - 1];
                            let node_index = self.nodi.iter().position(|node| node.node_id == node_id);
                            if let Some(node_idx) = node_index {
                                let node = self.nodi[node_idx].clone();
                                match node.node_type {
                                    // If the node is a server, send the packet to the server channel
                                    SimulationControllerNodeType::SERVER { .. } => {
                                        if let Some(channel) = self.sc_server_channels.get(&node.node_id) {
                                            channel.send(ServerCommand::ControllerShortcut(packet.clone()))
                                                .expect("Impossible to send ControllerShortcut to Server");
                                        }
                                    }
                                    // If the node is a client, send the packet to the client channel
                                    SimulationControllerNodeType::CLIENT { .. } => {
                                        if let Some(channel) = self.sc_client_channels.get(&node.node_id) {
                                            channel.send(ClientCommand::ControllerShortcut(packet.clone()))
                                                .expect("Impossible to send ControllerShortcut to Client");
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                // Handle client-related events
                Event::ClientEvent(client_event) => {
                    match client_event {
                        // Handle the event when a client sends a packet
                        ClientEvent::PacketSent(packet) => {
                            match packet.clone().pack_type {
                                // Notify about a message fragment sent by the client
                                PacketType::MsgFragment(_) => {
                                    self.print_client_notify(format!("Client {} sent a fragment to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // Notify about an ACK sent by the client
                                PacketType::Ack(_) => {
                                    self.print_client_notify(format!("Client {} sent an ACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // Handle different types of NACKs sent by the client
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        // Notify about an ErrorInRouting NACK
                                        NackType::ErrorInRouting(_) => {
                                            self.print_client_notify(format!("Client {} sent an ErrorInRouting NACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about a DestinationIsDrone NACK
                                        NackType::DestinationIsDrone => {
                                            self.print_client_notify(format!("Client {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about a Dropped NACK
                                        NackType::Dropped => {
                                            self.print_client_notify(format!("Client {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about an UnexpectedRecipient NACK
                                        NackType::UnexpectedRecipient(_) => {
                                            self.print_client_notify(format!("Client {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        ClientEvent::Route(route) => {
                            self.print_client_notify(format!("Client {} send {:?}", route[0], route), ui);
                        }
                        ClientEvent::DebugMessage(a, mess) => {
                            self.print_client_notify(format!("{:?}", mess), ui);
                        }
                        _ => {}
                    }
                }

                // Handle server-related events
                Event::ServerEvent(server_event) => {
                    match server_event {
                        // Handle the event when a server sends a packet
                        ServerEvent::PacketSent(packet) => {
                            match packet.clone().pack_type {
                                // Notify about a message fragment sent by the server
                                PacketType::MsgFragment(_) => {
                                    self.print_server_notify(format!("Server {} sent a fragment to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // Notify about an ACK sent by the server
                                PacketType::Ack(_) => {
                                    self.print_server_notify(format!("Server {} sent an ACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                                // Handle different types of NACKs sent by the server
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        // Notify about an ErrorInRouting NACK
                                        NackType::ErrorInRouting(_) => {
                                            self.print_server_notify(format!("Server {} sent an ErrorInRouting NACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about a DestinationIsDrone NACK
                                        NackType::DestinationIsDrone => {
                                            self.print_server_notify(format!("Server {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about a Dropped NACK
                                        NackType::Dropped => {
                                            self.print_server_notify(format!("Server {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                        // Notify about an UnexpectedRecipient NACK
                                        NackType::UnexpectedRecipient(_) => {
                                            self.print_server_notify(format!("Server {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        ServerEvent::Route(route) => {
                            self.print_client_notify(format!("Server {} send {:?}", route[0], route), ui);
                        }
                        ServerEvent::DebugMessage(a, mess) => {
                            self.print_client_notify(format!("{:?}", mess), ui);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Helper function to display a drone-related notification in the UI.
    pub fn print_drone_notify(&self, text: String, ui: &mut egui::Ui) {
<<<<<<< Updated upstream
        ui.vertical(|ui| {
            ui.add_space(8.);
=======
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.add_space(8.0);
>>>>>>> Stashed changes
            ui.label(RichText::new(text).size(15.0).color(Color32::LIGHT_BLUE));
        });
        ui.add_space(4.0);
    }

    /// Helper function to display a client-related notification in the UI.
    pub fn print_client_notify(&self, text: String, ui: &mut egui::Ui) {
<<<<<<< Updated upstream
        ui.vertical(|ui| {
            ui.add_space(8.);
=======
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.add_space(8.0);
>>>>>>> Stashed changes
            ui.label(RichText::new(text).size(15.0).color(Color32::LIGHT_GREEN));
        });
        ui.add_space(4.0);
    }

    /// Helper function to display a server-related notification in the UI.
    pub fn print_server_notify(&self, text: String, ui: &mut egui::Ui) {
<<<<<<< Updated upstream
        ui.vertical(|ui| {
            ui.add_space(8.);
=======
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.add_space(8.0);
>>>>>>> Stashed changes
            ui.label(RichText::new(text).size(15.0).color(Color32::LIGHT_RED));
        });
        ui.add_space(4.0);
    }
}