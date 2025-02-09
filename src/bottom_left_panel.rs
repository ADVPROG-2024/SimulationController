use std::fmt::Debug;
use dronegowski_utils::hosts::{ClientEvent, ServerEvent};
use eframe::egui;
use wg_2024::controller::DroneEvent;
use wg_2024::packet::{NackType, PacketType};
use crate::DronegowskiSimulationController;
use crate::sc_utils::Event;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        ui.heading("NOTIFICHE".to_string());
        for elem in &self.panel.bottom_left_panel.event{
            match elem {
                Event::DroneEvent(drone_event) => {
                    match drone_event {
                        DroneEvent::PacketSent (packet) => {
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            let node_id_sender = packet.routing_header.hops[0];

                            match packet.clone().pack_type {
                                PacketType::MsgFragment(fragment) => {
                                    ui.label(format!("Drone {} received fragment {} of {} from Node {}", node_id_receiver, fragment.fragment_index, fragment.total_n_fragments, node_id_sender));
                                }
                                PacketType::Ack(_) => {
                                    ui.label(format!("Drone {} received an ACK from Node {}", node_id_receiver, node_id_sender));
                                }
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        NackType::ErrorInRouting(_) => {
                                            ui.label(format!("Drone {} received an ErrorInRouting NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::DestinationIsDrone => {
                                            ui.label(format!("Drone {} received a DestinationIsDrone NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::Dropped => {
                                            ui.label(format!("Drone {} received a Dropped NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::UnexpectedRecipient(_) => {
                                            ui.label(format!("Drone {} received an UnexpectedRecipient NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                    }
                                }
                                PacketType::FloodRequest(_) => {
                                    ui.label(format!("Drone {} received a FloodRequest from Node {}", node_id_receiver, node_id_sender));
                                }
                                PacketType::FloodResponse(_) => {
                                    ui.label(format!("Drone {} received a FloodResponse from Node {}", node_id_receiver, node_id_sender));
                                }
                            }
                        }
                        DroneEvent::PacketDropped(packet) => {
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Drone {} dropped a Packet", node_id_receiver));
                        }
                        DroneEvent::ControllerShortcut(packet) => {
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            match packet.pack_type {
                                PacketType::Ack(_) => {
                                    ui.label(format!("Drone {} directed send to Simulation Controller an ACK", node_id_receiver));
                                }
                                PacketType::Nack(_) => {
                                    ui.label(format!("Drone {} directed send to Simulation Controller a NACK", node_id_receiver));
                                }
                                PacketType::FloodResponse(_) => {
                                    ui.label(format!("Drone {} directed send to Simulation Controller a FloodResponse", node_id_receiver));
                                }
                                _ => {}
                            }
                        }
                    }


                }
                Event::ClientEvent (client_event) => {
                    match client_event {
                        ClientEvent::PacketSent (packet) => {
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            let node_id_sender = packet.routing_header.hops[0];

                            match packet.clone().pack_type {
                                PacketType::MsgFragment(fragment) => {
                                    ui.label(format!("Client {} received fragment {} of {} from Node {}", node_id_receiver, fragment.fragment_index, fragment.total_n_fragments, node_id_sender));
                                }
                                PacketType::Ack(_) => {
                                    ui.label(format!("Client {} received an ACK from Node {}", node_id_receiver, node_id_sender));
                                }
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        NackType::ErrorInRouting(_) => {
                                            ui.label(format!("Client {} received an ErrorInRouting NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::DestinationIsDrone => {
                                            ui.label(format!("Client {} received a DestinationIsDrone NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::Dropped => {
                                            ui.label(format!("Client {} received a Dropped NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::UnexpectedRecipient(_) => {
                                            ui.label(format!("Client {} received an UnexpectedRecipient NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                    }
                                }
                                PacketType::FloodRequest(_) => {
                                    ui.label(format!("Client {} received a FloodRequest from Node {}", node_id_receiver, node_id_sender));
                                }
                                PacketType::FloodResponse(_) => {
                                    ui.label(format!("Client {} received a FloodResponse from Node {}", node_id_receiver, node_id_sender));
                                }
                            }
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
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            let node_id_sender = packet.routing_header.hops[0];
                            match packet.clone().pack_type {
                                PacketType::MsgFragment(fragment) => {
                                    ui.label(format!("Server {} received fragment {} of {} from Node {}", node_id_receiver, fragment.fragment_index, fragment.total_n_fragments, node_id_sender));
                                }
                                PacketType::Ack(_) => {
                                    ui.label(format!("Server {} received an ACK from Node {}", node_id_receiver, node_id_sender));
                                }
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        NackType::ErrorInRouting(_) => {
                                            ui.label(format!("Server {} received an ErrorInRouting NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::DestinationIsDrone => {
                                            ui.label(format!("Server {} received a DestinationIsDrone NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::Dropped => {
                                            ui.label(format!("Server {} received a Dropped NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                        NackType::UnexpectedRecipient(_) => {
                                            ui.label(format!("Server {} received an UnexpectedRecipient NACK from Node {}", node_id_receiver, node_id_sender));
                                        }
                                    }
                                }
                                PacketType::FloodRequest(_) => {
                                    ui.label(format!("Server {} received a FloodRequest from Node {}", node_id_receiver, node_id_sender));
                                }
                                PacketType::FloodResponse(_) => {
                                    ui.label(format!("Server {} received a FloodResponse from Node {}", node_id_receiver, node_id_sender));
                                }
                            }
                        }
                        ServerEvent::MessageReceived(_) => {}
                    }
                }
            }
        }
    }
}
