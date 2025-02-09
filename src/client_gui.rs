use std::collections::HashMap;
use crossbeam_channel::Sender;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerType, ClientType};
use eframe::egui;
use eframe::egui::{Color32, RichText, Layout, Align, WidgetText, Sense};
use wg_2024::network::NodeId;

pub fn client_gui(node_id: &NodeId, ctx: &egui::Context, popups_to_remove: &mut Vec<NodeId>, available_servers: &Vec<NodeId>, sc_client_channels: &HashMap<NodeId, Sender<ClientCommand>>, client_type: ClientType) {
    // --- State Management (Using persistent storage) ---
    let id = egui::Id::new(node_id).with("client_gui_state");

    // Use a helper function to simplify state management.  Crucially, use FnMut
    fn get_set_state<T: Clone + 'static + Send + Sync>(ctx: &egui::Context, id: egui::Id, default: T, ) -> (T, impl FnMut(T) + '_) {
        let current_value = ctx.data_mut(|data| data.get_temp_mut_or(id, default.clone()).clone());
        let set_value = move |new_value: T| {
            ctx.data_mut(|data| data.insert_temp(id, new_value));
        };
        (current_value, set_value)
    }

    let (mut selected_option, mut set_selected_option) = get_set_state(ctx, id.with("selected_option"), "Seleziona un'opzione".to_string());
    let (mut file_id_str, mut set_file_id_str) = get_set_state(ctx, id.with("file_id"), "".to_string());
    let (mut media_id_str, mut set_media_id_str) = get_set_state(ctx, id.with("media_id"), "".to_string());
    let (mut recipient_id_str, mut set_recipient_id_str) = get_set_state(ctx, id.with("recipient_id"), "".to_string());
    let (mut message_str, mut set_message_str) = get_set_state(ctx, id.with("message"), "".to_string());
    let (mut selected_server_id, mut set_selected_server_id) = get_set_state(ctx, id.with("selected_server_id"), 0);
    let (mut server_type, mut set_server_type) = get_set_state(ctx, id.with("server_type"), None::<(NodeId, ServerType)>);
    let (mut client_list, mut set_client_list) = get_set_state(ctx, id.with("client_list"), None::<(NodeId, Vec<NodeId>)>);
    let (mut files_list, mut set_files_list) = get_set_state(ctx, id.with("files_list"), None::<(NodeId, Vec<(u64, String)>)>);
    let (mut received_file, mut set_received_file) = get_set_state(ctx, id.with("received_file"), None::<(NodeId, String)>);
    let (mut received_media, mut set_received_media) = get_set_state(ctx, id.with("received_media"), None::<(NodeId, Vec<u8>)>);
    let (mut message_from, mut set_message_from) = get_set_state(ctx, id.with("message_from"), None::<(NodeId, NodeId, String)>);
    let (mut registration_result, mut set_registration_result) = get_set_state(ctx, id.with("registration_result"), None::<(NodeId, bool)>);
    let (mut status_messages, mut set_status_messages) = get_set_state(ctx, id.with("status_messages"), Vec::<String>::new());
    let (mut is_request_pending, mut set_is_request_pending) = get_set_state(ctx, id.with("request_pending"), false);
    let (mut connected_server, mut set_connected_server) = get_set_state(ctx, id.with("connected_server"), None::<NodeId>);

    // --- Styling ---
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = Color32::from_rgb(248, 248, 248);
    style.visuals.window_stroke = egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200));
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);
    style.spacing.item_spacing = egui::Vec2::new(8.0, 8.0);
    ctx.set_style(style);
    let text_color = ctx.style().visuals.text_color();

    egui::Window::new(format!("Client ({:?}): {}", client_type, node_id))
        .collapsible(false)
        .resizable(true)
        .frame(egui::Frame::window(&ctx.style()))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Node Details: Node ID:{}", node_id)).size(16.0).color(text_color));
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    let close_button = ui.add(
                        egui::Button::new(RichText::new("X").size(14.0).color(Color32::WHITE))
                            .fill(Color32::RED)
                    );
                    if close_button.clicked() {
                        popups_to_remove.push(*node_id);
                    }
                    close_button.on_hover_text(RichText::new("Close").color(Color32::BLACK));
                });
            });

            ui.separator();
            ui.add_space(5.0);

            // Refresh button (with loading indicator)
            // ui.horizontal(|ui| {
            //     let refresh_button = ui.add_enabled(
            //         !is_request_pending,
            //         egui::Button::new(
            //             if is_request_pending {
            //                 RichText::new("Refreshing...").size(14.0)
            //             } else {
            //                 RichText::new("Refresh Topology").size(14.0)
            //             }
            //         )
            //     );
            //
            //     if refresh_button.clicked() {
            //         if let Some(client_sender) = sc_client_channels.get(node_id) {
            //             if let Err(e) = client_sender.send(ClientCommand::RequestNetworkDiscovery) {
            //                 log::error!("Failed to send discovery request: {:?}", e);
            //                 let mut current_messages = status_messages.clone();
            //                 current_messages.push(format!("Error requesting discovery: {:?}", e));
            //                 set_status_messages(current_messages);
            //                 // Critically, reset is_request_pending on error
            //                 set_is_request_pending(false);
            //
            //             } else {
            //                 let mut current_messages = status_messages.clone();
            //                 current_messages.push("Sent network discovery request.".to_string());
            //                 set_status_messages(current_messages);
            //                 set_is_request_pending(true);
            //             }
            //         }
            //     }
            //     if is_request_pending {
            //         ui.spinner(); // Show a spinner while refreshing
            //     }
            // });


            ui.label(RichText::new("Choose an action:").color(text_color));

            // --- Conditional Options based on ClientType ---
            ui.horizontal(|ui| {
                ui.set_width(ui.available_width() * 0.6);
                egui::ComboBox::from_label("Options")
                    .selected_text(RichText::new(selected_option.clone()).color(text_color))
                    .show_ui(ui, |ui| {
                        let options = match client_type.clone() {
                            ClientType::WebBrowsers => vec![
                                "ServerType",
                                "FileList",
                                "File",
                                "Media",
                            ],
                            ClientType::ChatClients => vec![
                                "ServerType",
                                "ClientList",
                                "RegistrationToChat",
                                "MessageFor",
                            ],
                        };

                        for option in options {
                            let selectable_label = ui.selectable_value(&mut selected_option, option.to_string(), option);
                            if selectable_label.clicked() {
                                set_selected_option(selected_option.clone());
                            }
                            selectable_label.on_hover_text(RichText::new(format!("Select Option {}", option)).color(Color32::BLACK));
                        }
                    });

            });


            ui.add_space(10.0);
            // --- Server Selection and Connection Status ---
            ui.horizontal(|ui| {
                ui.label(RichText::new("Select Target Server:").color(text_color));

                egui::ComboBox::from_label("Server")
                    .selected_text(RichText::new(format!("Server {}", selected_server_id)).color(text_color))
                    .show_ui(ui, |ui| {
                        for server_id in available_servers {
                            let selectable_label = ui.selectable_value(&mut selected_server_id, *server_id, format!("Server {}", server_id));
                            if selectable_label.clicked() {
                                set_selected_server_id(*server_id);
                            }
                            selectable_label.on_hover_text(RichText::new(format!("Select server {}", server_id)).color(Color32::BLACK));
                        }
                    });
                // Connection Status Indicator (using color)
                let connection_status_color = match connected_server {
                    Some(server_id) if server_id == selected_server_id => Color32::GREEN,
                    Some(_) => Color32::YELLOW,
                    None => Color32::RED,
                };

                ui.label(RichText::new("●").color(connection_status_color).size(20.0));
            });

            ui.label(RichText::new(format!("Selected Option: {}", selected_option)).color(text_color));
            ui.add_space(5.0);

            // --- Conditional Input Fields (adjusted for ClientType) ---
            ui.vertical(|ui| {
                match client_type.clone() {
                    ClientType::WebBrowsers => {
                        if selected_option == "File" {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("File ID:").color(text_color));
                                let file_id_edit = ui.add(egui::TextEdit::singleline(&mut file_id_str).desired_width(100.0));
                                set_file_id_str(file_id_str.clone());
                                if !file_id_str.is_empty() && file_id_str.parse::<u64>().is_err() {
                                    ui.label(RichText::new("Invalid number!").color(Color32::RED));
                                    file_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED));
                                }
                            });
                        } else if selected_option == "Media" {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Media ID:").color(text_color));
                                let media_id_edit = ui.add(egui::TextEdit::singleline(&mut media_id_str).desired_width(100.0));
                                set_media_id_str(media_id_str.clone());
                                if !media_id_str.is_empty() && media_id_str.parse::<u64>().is_err() {
                                    ui.label(RichText::new("Invalid number!").color(Color32::RED));
                                    media_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED));
                                }
                            });
                        }
                    }
                    ClientType::ChatClients => {
                        if selected_option == "MessageFor" {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Recipient ID:").color(text_color));
                                let recipient_id_edit = ui.add(egui::TextEdit::singleline(&mut recipient_id_str).desired_width(100.0));
                                set_recipient_id_str(recipient_id_str.clone());
                                if !recipient_id_str.is_empty() && recipient_id_str.parse::<u64>().is_err()
                                {
                                    ui.label(RichText::new("Invalid number!").color(Color32::RED));
                                    recipient_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED));
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Message:").color(text_color));
                                ui.add(egui::TextEdit::multiline(&mut message_str).desired_rows(3));
                                set_message_str(message_str.clone());
                            });
                        }
                    }
                }
            });

            ui.add_space(10.0);

            // --- Conditional Send Button Logic (adjusted for ClientType) ---
            let can_send = match (client_type.clone(), selected_option.as_str()) {
                (ClientType::WebBrowsers, "ServerType" | "FileList") => selected_server_id != 0,
                (ClientType::WebBrowsers, "File") => !file_id_str.is_empty() && file_id_str.parse::<u64>().is_ok() && selected_server_id != 0,
                (ClientType::WebBrowsers, "Media") => !media_id_str.is_empty() && media_id_str.parse::<u64>().is_ok() && selected_server_id != 0,

                (ClientType::ChatClients, "ServerType" | "ClientList" | "RegistrationToChat") => selected_server_id != 0,
                (ClientType::ChatClients, "MessageFor") => {
                    !recipient_id_str.is_empty()
                        && recipient_id_str.parse::<u64>().is_ok()
                        && !message_str.is_empty() && selected_server_id != 0
                }
                _ => false,
            };

            let send_button = ui.add_enabled(
                can_send && !is_request_pending,
                egui::Button::new(
                    if is_request_pending {
                        RichText::new("Sending...").size(14.0).color(Color32::WHITE)
                    } else {
                        RichText::new("Send").size(14.0).color(if can_send{Color32::WHITE} else {Color32::GRAY})
                    }
                )
                    .fill(if can_send && !is_request_pending{ ctx.style().visuals.widgets.active.bg_fill} else {Color32::from_rgb(200, 200, 200)}),
            );


            if send_button.clicked() {
                // --- Conditional Command Creation (adjusted for ClientType) ---
                let command = match (client_type.clone(), selected_option.as_str()) {
                    (ClientType::WebBrowsers, "ServerType") => ClientCommand::ServerType(selected_server_id),
                    (ClientType::WebBrowsers, "FileList") => ClientCommand::FilesList(selected_server_id),
                    (ClientType::WebBrowsers, "File") => {
                        let file_id = file_id_str.parse::<u64>().expect("File ID should be valid");
                        ClientCommand::File(selected_server_id, file_id)
                    }
                    (ClientType::WebBrowsers, "Media") => {
                        let media_id = media_id_str.parse::<u64>().expect("Media ID should be valid");
                        ClientCommand::Media(selected_server_id, media_id)
                    }
                    (ClientType::ChatClients, "ServerType") => ClientCommand::ServerType(selected_server_id),
                    (ClientType::ChatClients, "ClientList") => ClientCommand::ClientList(selected_server_id),
                    (ClientType::ChatClients, "RegistrationToChat") => ClientCommand::RegistrationToChat(selected_server_id),
                    (ClientType::ChatClients, "MessageFor") => {
                        let recipient_id = recipient_id_str.parse::<u64>().expect("Recipient ID should be valid");
                        ClientCommand::MessageFor(selected_server_id, recipient_id as NodeId, message_str.clone())
                    }

                    _ => panic!("Invalid selected option / client type combination"),
                };

                if let Some(client_sender) = sc_client_channels.get(node_id) {
                    set_is_request_pending(true);

                    let mut current_messages_send = status_messages.clone();
                    current_messages_send.push(format!("Sending command: {:?}", command));
                    set_status_messages(current_messages_send);

                    if let Err(e) = client_sender.send(command){
                        log::error!("Failed to send command to client");
                        let mut current_messages_error = status_messages.clone();
                        current_messages_error.push(format!("Error sending command: {}", e));
                        set_status_messages(current_messages_error);
                        // Critically, reset is_request_pending on error
                        set_is_request_pending(false);
                    }
                } else {
                    log::error!("No communication channel found for client {}", node_id);
                    let mut current_messages_no_channel = status_messages.clone();
                    current_messages_no_channel.push(format!("No communication channel for client {}", node_id));
                    set_status_messages(current_messages_no_channel);
                    // Also reset here, since we couldn't even send
                    set_is_request_pending(false);
                }
            }
            send_button.on_hover_text(RichText::new("Send request to node").color(Color32::BLACK));


            // --- Server Response Display Area ---
            ui.add_space(20.0);
            ui.separator();
            ui.label(RichText::new("Server Responses:").size(16.0).color(text_color));

            ui.vertical(|ui| {
                if let Some((server_id, st)) = server_type {
                    ui.label(RichText::new(format!("Server Type (from {}): {:?}", server_id, st)).color(text_color));
                }
                if let Some((server_id, cl)) = &client_list {
                    ui.label(RichText::new(format!("Client List (from {}): {:?}", server_id, cl)).color(text_color));
                }
                if let Some((server_id, fl)) = &files_list {
                    ui.label(RichText::new(format!("Files List (from {}):", server_id)).color(text_color));
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (file_id, file_name) in fl {
                            ui.label(RichText::new(format!("ID: {}, Name: {}", file_id, file_name)).color(text_color));
                        }
                    });
                }

                if let Some((server_id, file)) = &received_file {
                    ui.label(RichText::new(format!("Received file (from {}):", server_id)).color(text_color));
                    let file_label = ui.add(egui::Label::new(RichText::new(file).color(text_color)).sense(Sense::click()));
                    if file_label.clicked() {
                        ui.output_mut(|o| o.copied_text = file.clone());
                    }
                }

                if let Some((server_id, media)) = &received_media {
                    ui.label(RichText::new(format!("Received media (from {}): dim {}", server_id, media.len())).color(text_color));
                }

                if let Some((server_id, from_id, message)) = &message_from {
                    ui.label(RichText::new(format!("Received message (from {}):", server_id)).color(text_color));
                    ui.label(RichText::new(format!("{:?} -> {}", from_id, message)).color(text_color));
                }

                if let Some((server_id, result)) = registration_result {
                    let text = if result { "Registration OK" } else { "Registration Error" };
                    let color = if result { Color32::GREEN } else { Color32::RED };
                    ui.label(RichText::new(format!("Registration Result (from {}): {}", server_id, text)).color(color));
                }
            });

            // Status Messages Display
            ui.add_space(10.0);
            ui.separator();
            ui.label(RichText::new("Status Messages:").size(14.0).color(text_color));
            egui::ScrollArea::vertical()
                .max_height(100.0)
                .show(ui, |ui| {
                    for msg in &status_messages {
                        ui.label(RichText::new(msg).color(text_color));
                    }
                });
        });
}