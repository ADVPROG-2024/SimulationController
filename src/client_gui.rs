use std::collections::HashMap;
use crossbeam_channel::Sender;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerMessages, ServerType};
use eframe::egui;
use eframe::egui::{Color32, RichText, Layout, Align, WidgetText};
use wg_2024::network::NodeId;

pub fn client_gui(node_id: &NodeId, ctx: &egui::Context, popups_to_remove: &mut Vec<NodeId>, available_servers: &Vec<NodeId>, sc_client_channels: &HashMap<NodeId, Sender<ClientCommand>>) {
    // --- State Management ---
    let id = egui::Id::new(node_id).with("client_gui_state");
    let mut selected_option = ctx.data_mut(|data| {
        data.get_temp_mut_or(id.with("selected_option"), "Seleziona un'opzione".to_string())
            .clone()
    });
    let mut file_id_str = ctx.data_mut(|data| data.get_temp_mut_or(id.with("file_id"), "".to_string()).clone());
    let mut media_id_str = ctx.data_mut(|data| data.get_temp_mut_or(id.with("media_id"), "".to_string()).clone());
    let mut recipient_id_str = ctx.data_mut(|data| data.get_temp_mut_or(id.with("recipient_id"), "".to_string()).clone());
    let mut message_str = ctx.data_mut(|data| data.get_temp_mut_or(id.with("message"), "".to_string()).clone());
    let mut selected_server_id = ctx.data_mut(|data| {
        data.get_temp_mut_or(id.with("selected_server_id"), 0).clone() // Default to 0 (or some other invalid ID)
    });

    // --- GUI State for Server Responses ---
    let mut server_type = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, ServerType)>>(id.with("server_type"), None).clone());
    let mut client_list = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, Vec<NodeId>)>>(id.with("client_list"), None).clone());
    let mut files_list = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, Vec<(u64, String)>)>>(id.with("files_list"), None).clone());
    let mut received_file = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, String)>>(id.with("received_file"), None).clone());
    let mut received_media = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, Vec<u8>)>>(id.with("received_media"), None).clone());
    let mut message_from = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, NodeId, String)>>(id.with("message_from"), None).clone());
    let mut registration_result = ctx.data_mut(|data| data.get_temp_mut_or::<Option<(NodeId, bool)>>(id.with("registration_result"), None).clone()); // true for Ok, false for Error

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

    egui::Window::new(format!("Client: {}", node_id))
        .collapsible(false)
        .resizable(true)
        .frame(egui::Frame::window(&ctx.style()))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Node Details: {}", node_id)).size(16.0).color(text_color));
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

            ui.label(RichText::new("Choose an action:").color(text_color));

            egui::ComboBox::from_label("Options")
                .selected_text(RichText::new(selected_option.clone()).color(text_color))
                .show_ui(ui, |ui| {
                    let options = [
                        "ServerType",
                        "FileList",
                        "File",
                        "Media",
                        "ClientList",
                        "RegistrationToChat",
                        "MessageFor",
                    ];
                    for option in &options {
                        let selectable_label = ui.selectable_value(&mut selected_option, option.to_string(), *option);
                        if selectable_label.clicked() {
                            ctx.data_mut(|data| data.insert_temp(id.with("selected_option"), selected_option.clone()));
                        }
                        selectable_label.on_hover_text(RichText::new(format!("Select Option {}", option)).color(Color32::BLACK));
                    }
                });

            ui.add_space(10.0);
            ui.label(RichText::new("Select Target Server:").color(text_color));
            egui::ComboBox::from_label("Server")
                .selected_text(RichText::new(format!("Server {}", selected_server_id)).color(text_color))
                .show_ui(ui, |ui| {
                    for server_id in available_servers {
                        let selectable_label = ui.selectable_value(&mut selected_server_id, *server_id, format!("Server {}", server_id));
                        if selectable_label.clicked() {
                            ctx.data_mut(|data| data.insert_temp(id.with("selected_server_id"), selected_server_id.clone()));
                        }
                    }
                });

            ui.label(RichText::new(format!("Selected Option: {}", selected_option)).color(text_color));
            ui.add_space(5.0);

            ui.vertical(|ui| {
                if selected_option == "File" {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("File ID:").color(text_color));
                        let file_id_edit = ui.add(egui::TextEdit::singleline(&mut file_id_str));
                        ctx.data_mut(|data| data.insert_temp(id.with("file_id"), file_id_str.clone()));

                        if !file_id_str.is_empty() && file_id_str.parse::<u64>().is_err() {
                            ui.label(RichText::new("Invalid number!").color(Color32::RED));
                            file_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED));
                        }
                    });
                } else if selected_option == "Media" {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Media ID:").color(text_color));
                        let media_id_edit = ui.add(egui::TextEdit::singleline(&mut media_id_str));
                        ctx.data_mut(|data| data.insert_temp(id.with("media_id"), media_id_str.clone()));

                        if !media_id_str.is_empty() && media_id_str.parse::<u64>().is_err() {
                            ui.label(RichText::new("Invalid number!").color(Color32::RED));
                            media_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED));
                        }
                    });
                } else if selected_option == "MessageFor" {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Recipient ID:").color(text_color));
                        let recipient_id_edit = ui.add(egui::TextEdit::singleline(&mut recipient_id_str));
                        ctx.data_mut(|data| data.insert_temp(id.with("recipient_id"), recipient_id_str.clone()));

                        if !recipient_id_str.is_empty() && recipient_id_str.parse::<u64>().is_err() {
                            ui.label(RichText::new("Invalid number!").color(Color32::RED));
                            recipient_id_edit.on_hover_text(RichText::new("Invalid Input").color(Color32::RED));
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Message:").color(text_color));
                        ui.add(egui::TextEdit::multiline(&mut message_str));
                        ctx.data_mut(|data| data.insert_temp(id.with("message"), message_str.clone()));
                    });
                }
            });

            ui.add_space(10.0);

            let can_send = match selected_option.as_str() {
                "ServerType" | "FileList" | "ClientList" | "RegistrationToChat" =>  selected_server_id != 0,
                "File" => !file_id_str.is_empty() && file_id_str.parse::<u64>().is_ok() && selected_server_id != 0,
                "Media" => !media_id_str.is_empty() && media_id_str.parse::<u64>().is_ok() && selected_server_id != 0,
                "MessageFor" => {
                    !recipient_id_str.is_empty()
                        && recipient_id_str.parse::<u64>().is_ok()
                        && !message_str.is_empty() && selected_server_id != 0
                }
                _ => false,
            };

            let send_button = ui.add_enabled(
                can_send,
                egui::Button::new(RichText::new("Send").size(14.0).color(if can_send{Color32::WHITE} else {Color32::GRAY}))
                    .fill(if can_send {ctx.style().visuals.widgets.active.bg_fill} else {Color32::from_rgb(200, 200, 200)}),
            );
            if send_button.clicked() {
                let command = match selected_option.as_str() {
                    "ServerType" => ClientCommand::ServerType(selected_server_id),
                    "FileList" => ClientCommand::FilesList(selected_server_id),
                    "File" => {
                        let file_id = file_id_str.parse::<u64>().expect("File ID should be valid");
                        ClientCommand::File(selected_server_id, file_id)
                    }
                    "Media" => {
                        let media_id = media_id_str.parse::<u64>().expect("Media ID should be valid");
                        ClientCommand::Media(selected_server_id, media_id)
                    }
                    "ClientList" => ClientCommand::ClientList(selected_server_id),
                    "RegistrationToChat" => ClientCommand::RegistrationToChat(selected_server_id),
                    "MessageFor" => {
                        let recipient_id = recipient_id_str.parse::<u64>().expect("Recipient ID should be valid");
                        ClientCommand::MessageFor(selected_server_id, recipient_id as NodeId, message_str.clone())
                    }
                    _ => panic!("Invalid selected option"),
                };

                if let Some(client_sender) = sc_client_channels.get(node_id) {
                    client_sender.send(command).expect("Failed to send command to client");
                } else {
                    log::error!("No communication channel found for client {}", node_id);
                }
            }
            send_button.on_hover_text(RichText::new("Send request to node").color(Color32::BLACK));

            // --- Server Response Display Area ---
            ui.add_space(20.0);
            ui.separator();
            ui.label(RichText::new("Server Responses:").size(16.0).color(text_color));

            ui.vertical(|ui| {  // Or ui.horizontal, depending on desired layout
                if let Some((server_id, st)) = server_type {
                    ui.label(RichText::new(format!("Server Type (from {}): {:?}", server_id, st)).color(text_color));
                }
                if let Some((server_id, cl)) = &client_list {
                    ui.label(RichText::new(format!("Client List (from {}): {:?}", server_id, cl)).color(text_color));
                }
                if let Some((server_id, fl)) = &files_list{
                    ui.label(RichText::new(format!("Files List (from {}): {:?}", server_id, fl)).color(text_color));
                }
                if let Some((server_id, file)) = &received_file{
                    ui.label(RichText::new(format!("Received file (from {}): {:?}", server_id, file)).color(text_color));
                }
                if let Some((server_id, media)) = &received_media{
                    ui.label(RichText::new(format!("Received media (from {}): dim {}", server_id, media.len())).color(text_color));
                }
                if let Some((server_id, from_id, message)) = &message_from{
                    ui.label(RichText::new(format!("Received message (from {}): {:?} -> {}", server_id, from_id, message)).color(text_color));
                }

                if let Some((server_id, result)) = registration_result {
                    let text = if result { "Registration OK" } else { "Registration Error" };
                    ui.label(RichText::new(format!("Registration Result (from {}): {}", server_id, text)).color(text_color));
                }
            });
        });
}