use std::collections::HashMap;
use crossbeam_channel::Sender;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerType, ClientType};
use eframe::egui;
use eframe::egui::{Color32, RichText, Layout, Align, WidgetText, Sense};
use wg_2024::network::NodeId;

pub fn client_gui(node_id: &NodeId, ctx: &egui::Context, popups_to_remove: &mut Vec<NodeId>, available_servers: &Vec<NodeId>, sc_client_channels: &HashMap<NodeId, Sender<ClientCommand>>, client_type: ClientType) {
    // --- State Management (Using persistent storage) ---
    let id = egui::Id::new(node_id).with("client_gui_state");

    // Helper function to simplify state management using temporary storage within egui context.
    // It retrieves or initializes state and provides a setter function.
    fn get_set_state<T: Clone + 'static + Send + Sync>(ctx: &egui::Context, id: egui::Id, default: T, ) -> (T, impl FnMut(T) + '_) {
        let current_value = ctx.data_mut(|data| data.get_temp_mut_or(id, default.clone()).clone());
        let set_value = move |new_value: T| {
            ctx.data_mut(|data| data.insert_temp(id, new_value));
        };
        (current_value, set_value)
    }

    // State variables for the GUI, managed using the helper function.
    let (mut selected_option, mut set_selected_option) = get_set_state(ctx, id.with("selected_option"), "Select an option".to_string());
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
    let (mut error_messages, mut set_error_messages) = get_set_state(ctx, id.with("error_messages"), Vec::<String>::new());

    // --- Styling ---
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = Color32::from_rgb(248, 248, 248); // Light background for windows
    style.visuals.window_stroke = egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)); // Light grey window border
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0); // Rounded corners for non-interactive widgets
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0); // Rounded corners for inactive widgets
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0); // Rounded corners for hovered widgets
    style.visuals.widgets.active.rounding = egui::Rounding::same(4.0); // Rounded corners for active widgets
    style.spacing.item_spacing = egui::Vec2::new(8.0, 8.0); // Standard spacing between items
    ctx.set_style(style);
    let text_color = ctx.style().visuals.text_color(); // Get default text color from style

    egui::Window::new(format!("Client ({:?}): {}", client_type, node_id)) // Window title includes client type and node ID
        .collapsible(false) // Window cannot be collapsed
        .resizable(true) // Window is resizable
        .frame(egui::Frame::window(&ctx.style())) // Use window frame style
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Node Details: Node ID:{}", node_id)).size(16.0).color(text_color)); // Display node details
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    let close_button = ui.add(
                        egui::Button::new(RichText::new("X").size(14.0).color(Color32::WHITE))
                            .fill(Color32::RED) // Red background for close button
                    );
                    if close_button.clicked() {
                        popups_to_remove.push(*node_id); // Add node ID to removal list when closed
                    }
                    close_button.on_hover_text(RichText::new("Close").color(Color32::BLACK)); // Hover text for close button
                });
            });

            ui.separator();
            ui.add_space(5.0);

            ui.label(RichText::new("Choose an action:").color(text_color)); // Label for action selection

            // --- Conditional Options based on ClientType ---
            ui.horizontal(|ui| {
                ui.set_width(ui.available_width() * 0.6); // Set combobox width to 60% of available width
                egui::ComboBox::from_label("Actions") // Combobox for action selection
                    .selected_text(RichText::new(selected_option.clone()).color(text_color)) // Display selected option
                    .show_ui(ui, |ui| {
                        let options = match client_type.clone() { // Define options based on client type
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
                            let selectable_label = ui.selectable_value(&mut selected_option, option.to_string(), option); // Create selectable labels for each option
                            if selectable_label.clicked() {
                                set_selected_option(selected_option.clone()); // Update selected option on click
                            }
                            selectable_label.on_hover_text(RichText::new(format!("Select Action {}", option)).color(Color32::BLACK)); // Hover text for options
                        }
                    });

            });


            ui.add_space(10.0);
            // --- Server Selection and Connection Status ---
            ui.horizontal(|ui| {
                ui.label(RichText::new("Select Target Server:").color(text_color)); // Label for server selection

                egui::ComboBox::from_label("Servers") // Combobox for server selection
                    .selected_text(RichText::new(format!("Server {}", selected_server_id)).color(text_color)) // Display selected server ID
                    .show_ui(ui, |ui| {
                        for server_id in available_servers {
                            let selectable_label = ui.selectable_value(&mut selected_server_id, *server_id, format!("Server {}", server_id)); // Create selectable labels for each server
                            if selectable_label.clicked() {
                                set_selected_server_id(*server_id); // Update selected server ID on click
                            }
                            selectable_label.on_hover_text(RichText::new(format!("Select server {}", server_id)).color(Color32::BLACK)); // Hover text for servers
                        }
                    });
            });

            ui.label(RichText::new(format!("Selected Action: {}", selected_option)).color(text_color)); // Display currently selected action
            ui.add_space(5.0);

            // --- Conditional Input Fields (adjusted for ClientType) ---
            ui.vertical(|ui| {
                match client_type.clone() {
                    ClientType::WebBrowsers => {
                        if selected_option == "File" {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("File ID:").color(text_color)); // Label for File ID input
                                let file_id_edit = ui.add(egui::TextEdit::singleline(&mut file_id_str).desired_width(100.0)); // Text input for File ID
                                set_file_id_str(file_id_str.clone()); // Update state with File ID string
                                if !file_id_str.is_empty() && file_id_str.parse::<u64>().is_err() { // Input validation for File ID
                                    ui.label(RichText::new("Invalid number!").color(Color32::RED)); // Error label for invalid input
                                    file_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED)); // Hover text for invalid input
                                }
                            });
                        } else if selected_option == "Media" {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Media ID:").color(text_color)); // Label for Media ID input
                                let media_id_edit = ui.add(egui::TextEdit::singleline(&mut media_id_str).desired_width(100.0)); // Text input for Media ID
                                set_media_id_str(media_id_str.clone()); // Update state with Media ID string
                                if !media_id_str.is_empty() && media_id_str.parse::<u64>().is_err() { // Input validation for Media ID
                                    ui.label(RichText::new("Invalid number!").color(Color32::RED)); // Error label for invalid input
                                    media_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED)); // Hover text for invalid input
                                }
                            });
                        }
                    }
                    ClientType::ChatClients => {
                        if selected_option == "MessageFor" {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Recipient ID:").color(text_color)); // Label for Recipient ID input
                                let recipient_id_edit = ui.add(egui::TextEdit::singleline(&mut recipient_id_str).desired_width(100.0)); // Text input for Recipient ID
                                set_recipient_id_str(recipient_id_str.clone()); // Update state with Recipient ID string
                                if !recipient_id_str.is_empty() && recipient_id_str.parse::<u64>().is_err() // Input validation for Recipient ID
                                {
                                    ui.label(RichText::new("Invalid number!").color(Color32::RED)); // Error label for invalid input
                                    recipient_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED)); // Hover text for invalid input
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Message:").color(text_color)); // Label for message input
                                ui.add(egui::TextEdit::multiline(&mut message_str).desired_rows(3)); // Multiline text input for message
                                set_message_str(message_str.clone()); // Update state with message string
                            });
                        }
                    }
                }
            });

            ui.add_space(10.0);

            // --- Conditional Send Button Logic (adjusted for ClientType) ---
            let can_send = match (client_type.clone(), selected_option.as_str()) { // Determine if send button should be enabled based on client type and selected action
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
                can_send && !is_request_pending, // Enable button only if conditions are met and no request is pending
                egui::Button::new(
                    if is_request_pending {
                        RichText::new("Sending...").size(14.0).color(Color32::WHITE) // "Sending..." text when request is pending
                    } else {
                        RichText::new("Send").size(14.0).color(if can_send{Color32::WHITE} else {Color32::GRAY}) // "Send" text, grayed out if disabled
                    }
                )
                    .fill(if can_send && !is_request_pending{ ctx.style().visuals.widgets.active.bg_fill} else {Color32::from_rgb(200, 200, 200)}), // Button fill color, different when disabled
            );


            if send_button.clicked() {
                // --- Conditional Command Creation (adjusted for ClientType) ---
                let command = match (client_type.clone(), selected_option.as_str()) { // Create ClientCommand based on client type and selected option
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

                    _ => panic!("Invalid selected option / client type combination"), // Panic for invalid combination (should not happen due to UI constraints)
                };

                if let Some(client_sender) = sc_client_channels.get(node_id) { // Get sender channel for this client
                    set_is_request_pending(true); // Set request pending flag

                    let mut current_messages_send = status_messages.clone();
                    current_messages_send.push(format!("Sending command: {:?}", command)); // Add status message for command sending
                    set_status_messages(current_messages_send);

                    if let Err(e) = client_sender.send(command){ // Send command to client
                        log::error!("Failed to send command to client");
                        let mut current_messages_error = status_messages.clone();
                        current_messages_error.push(format!("Error sending command: {}", e)); // Add status message for error
                        set_status_messages(current_messages_error);
                        // Critically, reset is_request_pending on error
                        set_is_request_pending(false); // Reset request pending flag on error
                    }
                } else {
                    log::error!("No communication channel found for client {}", node_id);
                    let mut current_messages_no_channel = status_messages.clone();
                    current_messages_no_channel.push(format!("No communication channel for client {}", node_id)); // Add status message for no channel
                    set_status_messages(current_messages_no_channel);
                    // Also reset here, since we couldn't even send
                    set_is_request_pending(false); // Reset request pending flag if no channel
                }
            }
            send_button.on_hover_text(RichText::new("Send request to node").color(Color32::BLACK)); // Hover text for send button


            // --- Server Response Display Area ---
            ui.add_space(20.0);
            ui.separator();
            ui.label(RichText::new("Server Responses:").size(16.0).color(text_color)); // Label for server responses

            ui.vertical(|ui| {
                if let Some((server_id, st)) = server_type {
                    ui.label(RichText::new(format!("Server Type (from {}): {:?}", server_id, st)).color(text_color)); // Display server type response
                }
                if let Some((server_id, cl)) = &client_list {
                    ui.label(RichText::new(format!("Client List (from {}): {:?}", server_id, cl)).color(text_color)); // Display client list response
                }
                if let Some((server_id, fl)) = &files_list {
                    ui.label(RichText::new(format!("Files List (from {}):", server_id)).color(text_color)); // Display files list response label
                    egui::ScrollArea::vertical().show(ui, |ui| { // Use scroll area for files list
                        for (file_id, file_name) in fl {
                            ui.label(RichText::new(format!("ID: {}, Name: {}", file_id, file_name)).color(text_color)); // Display each file in list
                        }
                    });
                }

                if let Some((server_id, file)) = &received_file {
                    ui.label(RichText::new(format!("Received file (from {}):", server_id)).color(text_color)); // Display received file label
                    let file_label = ui.add(egui::Label::new(RichText::new(file).color(text_color)).sense(Sense::click())); // Display received file content, clickable to copy
                    if file_label.clicked() {
                        ui.output_mut(|o| o.copied_text = file.clone()); // Copy file content to clipboard on click
                    }
                }

                if let Some((server_id, media)) = &received_media {
                    ui.label(RichText::new(format!("Received media (from {}): dim {}", server_id, media.len())).color(text_color)); // Display received media info
                }

                if let Some((server_id, from_id, message)) = &message_from {
                    ui.label(RichText::new(format!("Received message (from {}):", server_id)).color(text_color)); // Display received message label
                    ui.label(RichText::new(format!("{:?} -> {}", from_id, message)).color(text_color)); // Display sender and message content
                }

                if let Some((server_id, result)) = registration_result {
                    let text = if result { "Registration OK" } else { "Registration Error" };
                    let color = if result { Color32::GREEN } else { Color32::RED };
                    ui.label(RichText::new(format!("Registration Result (from {}): {}", server_id, text)).color(color)); // Display registration result with color coding
                }
            });

            // Status Messages Display
            ui.add_space(10.0);
            ui.separator();
            ui.label(RichText::new("Status Messages:").size(14.0).color(text_color)); // Label for status messages
            egui::ScrollArea::vertical()
                .max_height(100.0) // Set max height for status message scroll area
                .show(ui, |ui| {
                    for msg in &status_messages {
                        ui.label(RichText::new(msg).color(text_color)); // Display each status message
                    }
                });
        });
}