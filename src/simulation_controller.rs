use std::collections::HashMap;
use std::fmt::Debug;
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};
use wg_2024;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use dronegowski::Dronegowski;
use dronegowski_utils::functions::validate_network;
use dronegowski_utils::functions::ValidationError;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use eframe::egui;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::{NodeId, SourceRoutingHeader};
use dronegowski_utils::network::{Event, SimulationControllerNode, SimulationControllerNodeType};
use rolling_drone::RollingDrone;
use eframe::egui::accesskit::Node;
use eframe::egui::Color32;
use rustastic_drone::RustasticDrone;
use wg_2024::drone::Drone;
use wg_2024::packet::{Fragment, NodeType, Packet};
use wg_2024::packet::PacketType::MsgFragment;
use crate::client_gui::client_gui;
use crate::sc_utils::Panel;
use log::{debug, error, info, warn};

pub struct DronegowskiSimulationController<'a> {
    pub nodi: Vec<SimulationControllerNode>,
    pub sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
    pub sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
    pub sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
    pub sc_drone_event_send: Sender<DroneEvent>,
    pub sc_drone_event_recv: Receiver<DroneEvent>,
    pub sc_client_event_recv: Receiver<ClientEvent>,
    pub sc_server_event_recv: Receiver<ServerEvent>,
    pub packet_node_channels: HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
    pub handles: &'a mut Vec<JoinHandle<()>>,
    pub panel: Panel,
}

impl <'a>DronegowskiSimulationController<'a> {
    pub fn new(nodi: Vec<SimulationControllerNode>,
               sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
               sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
               sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
               sc_drone_event_send: Sender<DroneEvent>,
               sc_drone_event_recv: Receiver<DroneEvent>,
               sc_client_event_recv: Receiver<ClientEvent>,
               sc_server_event_recv: Receiver<ServerEvent>,
               packet_node_channels: HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
               mut handles: &mut Vec<JoinHandle<()>>,
    ){
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_fullscreen(true),
            ..Default::default()
        };

        eframe::run_native(
            "Simulation Controller",
            native_options,
            Box::new(|cc| Ok(Box::new(DronegowskiSimulationController::create(cc, nodi, sc_drone_channels, sc_client_channels, sc_server_channels, sc_drone_event_send, sc_drone_event_recv, sc_client_event_recv, sc_server_event_recv, packet_node_channels, &mut handles)))),
        ).expect("Error to run the Simulation Controller");
    }

    fn create(_cc: &eframe::CreationContext<'_>,
              nodi: Vec<SimulationControllerNode>,
              sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
              sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
              sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
              sc_drone_event_send: Sender<DroneEvent>,
              sc_drone_event_recv: Receiver<DroneEvent>,
              sc_client_event_recv: Receiver<ClientEvent>,
              sc_server_event_recv: Receiver<ServerEvent>,
              packet_node_channels: HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
              handles: &'a mut Vec<JoinHandle<()>>,
    ) -> Self {
        Self {
            nodi: nodi.clone(),
            sc_drone_channels,
            sc_client_channels,
            sc_server_channels,
            sc_drone_event_send,
            sc_drone_event_recv,
            sc_client_event_recv,
            sc_server_event_recv,
            packet_node_channels,
            handles,
            panel: Panel::default(),
        }
    }
}

impl eframe::App for DronegowskiSimulationController<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        loop {
            select! {
                recv(self.sc_drone_event_recv) -> drone_event_res => {
                    if let Ok(drone_event) = drone_event_res {
                        self.handle_drone_event(drone_event);
                    }
                },
                recv(self.sc_client_event_recv) -> client_event_res => {
                    if let Ok(client_event) = client_event_res {
                        self.handle_client_event(client_event.clone());
                        let client_id = match &client_event {
                            ClientEvent::PacketSent(_) => None,
                            ClientEvent::ServerTypeReceived(client_id, _, _) => Some(client_id),
                            ClientEvent::ClientListReceived(client_id, _, _) => Some(client_id),
                            ClientEvent::FilesListReceived(client_id, _, _) => Some(client_id),
                            ClientEvent::FileReceived(client_id, _, _) => Some(client_id),
                            ClientEvent::MediaReceived(client_id, _, _) => Some(client_id),
                            ClientEvent::MessageFromReceived(client_id, _, _, _) => Some(client_id),
                            ClientEvent::RegistrationOk(client_id, _) => Some(client_id),
                            ClientEvent::Error(client_id, _) => Some(client_id),
                            ClientEvent::MessageReceived(_) => None,
                            _ => {None}
                        };

                        if let Some(client_id) = client_id {
                            let id = egui::Id::new(client_id).with("client_gui_state");

                            match client_event {
                                ClientEvent::ServerTypeReceived(_client_id, server_id, server_type) => {
                                    ctx.data_mut(|data| data.insert_temp(id.with("server_type"), Some((server_id, server_type))));
                                    log::info!("Simulation Controller: Received ClientEvent::ServerTypeReceived");
                                }
                                ClientEvent::ClientListReceived(_client_id, server_id, clients) => {
                                    ctx.data_mut(|data| data.insert_temp(id.with("client_list"), Some((server_id, clients))));
                                }
                                ClientEvent::FilesListReceived(_client_id, server_id, files) => {
                                    ctx.data_mut(|data| data.insert_temp(id.with("files_list"), Some((server_id, files))));
                                }
                                ClientEvent::FileReceived(_client_id, server_id, file_data) => {
                                    ctx.data_mut(|data| data.insert_temp(id.with("received_file"), Some((server_id, file_data))));
                                }
                                ClientEvent::MediaReceived(_client_id, server_id, media_data) => {
                                    ctx.data_mut(|data| data.insert_temp(id.with("received_media"), Some((server_id, media_data))));
                                }
                                ClientEvent::MessageFromReceived(_client_id, server_id, from_id, message) => {
                                    ctx.data_mut(|data| data.insert_temp(id.with("message_from"), Some((server_id, from_id, message))));
                                }
                                ClientEvent::RegistrationOk(_client_id, server_id) => {
                                     ctx.data_mut(|data| data.insert_temp(id.with("registration_result"), Some((server_id, true))));
                                }
                                ClientEvent::Error(client_id, message) => {
                                     ctx.data_mut(|data| data.insert_temp(id.with("error"), Some((client_id, message))));
                                }
                                _ => {}
                            }
                            ctx.data_mut(|data| data.insert_temp(id.with("request_pending"), false));
                        }
                    }
                },
                recv(self.sc_server_event_recv) -> server_event_res => {
                    if let Ok(server_event) = server_event_res {
                        self.handle_server_event(server_event.clone());
                        let client_id = match &server_event {
                            ServerEvent::Error(_, client_id, _) => Some(client_id),
                            _ => {None}
                        };
                        if let Some(client_id) = client_id {
                            let id = egui::Id::new(client_id).with("client_gui_state");
                            match server_event {
                                ServerEvent::Error(_, client_id, message) => {
                                    info!("Simulation Controller: Received ServerEvent::Error {:?} ", client_id);
                                    ctx.data_mut(|data| data.insert_temp(id.with("error"), Some((client_id, message))));
                                }
                                _ => {}
                            }
                            ctx.data_mut(|data| data.insert_temp(id.with("request_pending"), false));
                        }
                    }
                },
                default => break, // Exit the loop if no events are ready
            }
        } // End of the select! loop

        ctx.request_repaint();


        egui::SidePanel::left("left_panel").resizable(false).exact_width(300.0).frame(egui::Frame::none()).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

            // Upper left part
            ui.vertical(|ui| {
                ui.set_height(ui.available_height() / 2.0 - 100.);
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    0.0,
                    Color32::LIGHT_GRAY
                );

                match &self.panel.central_panel.selected_node.clone(){
                    None => { self.upper_left_panel_default(ui); }
                    Some(node) => {
                        match node.node_type {
                            SimulationControllerNodeType::DRONE {..} => {
                                self.upper_left_panel_drone(ui, self.panel.central_panel.selected_node.clone().unwrap());
                            }
                            SimulationControllerNodeType::CLIENT {..} => {
                                self.upper_left_panel_client(ui, self.panel.central_panel.selected_node.clone().unwrap());
                            }
                            SimulationControllerNodeType::SERVER {..} => {
                                self.upper_left_panel_server(ui, self.panel.central_panel.selected_node.clone().unwrap());
                            }
                        }
                    }
                }
            });

            // Bottom left part
            ui.vertical(|ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    0.0,
                    Color32::DARK_GRAY,
                );
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.bottom_left_panel(ui);
                });
            });
        });


        egui::SidePanel::right("right_panel")
            .resizable(false)
            .exact_width(300.0)
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

                // Upper right part
                ui.vertical(|ui| {
                    ui.set_height(ui.available_height() / 2.0 + 150.0);
                    ui.painter().rect_filled(
                        ui.available_rect_before_wrap(),
                        0.0,
                        Color32::WHITE,
                    );

                    ui.push_id("upper_right_scroll", |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height())
                            .show(ui, |ui| {
                                self.upper_right_panel(ui);
                            });
                    });
                });

                // Bottom right part
                ui.vertical(|ui| {
                    ui.set_height(ui.available_height());
                    ui.painter().rect_filled(
                        ui.available_rect_before_wrap(),
                        0.0,
                        Color32::DARK_GRAY,
                    );

                    ui.push_id("bottom_right_scroll", |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height())
                            .show(ui, |ui| {
                                self.bottom_right_panel(ui);
                            });
                    });
                });
            });
        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            self.central_panel(ui, ctx);
        });

        let mut popups_to_remove = vec![];

        let available_servers: Vec<NodeId> = self.nodi.iter()
            .filter_map(|node| {
                if let SimulationControllerNodeType::SERVER { .. } = node.node_type {
                    Some(node.node_id)
                } else {
                    None
                }
            })
            .collect();
        for (node_id, node) in &self.panel.central_panel.active_popups {
            if let SimulationControllerNodeType::CLIENT { client_type, .. } = node.node_type.clone() {
                client_gui(node_id, &ctx.clone(), &mut popups_to_remove, &available_servers, &self.sc_client_channels, client_type); // Pass client_type
            }
        }
        for node_id in popups_to_remove {
            self.panel.central_panel.active_popups.remove(&node_id);
        }
    }
}

impl DronegowskiSimulationController<'_>{
    pub fn add_sender(&mut self, neighbour_id: NodeId){
        let current_node_id = self.panel.central_panel.selected_node.clone().unwrap().node_id;

        let current_node_index = self.nodi.iter().position(|node| node.node_id == current_node_id);
        let neighbour_index = self.nodi.iter().position(|node| node.node_id == neighbour_id);

        if let (Some(current_index), Some(neighbour_index)) = (current_node_index, neighbour_index) {
            let mut current_node = self.nodi[current_index].clone();
            let mut neighbour = self.nodi[neighbour_index].clone();

            current_node.neighbours.push(neighbour_id);
            neighbour.neighbours.push(current_node_id);

            let mut node_verification = self.nodi.clone();
            node_verification[current_index] = current_node.clone();
            node_verification[neighbour_index] = neighbour.clone();
            let result = validate_network(&node_verification);
            match result {
                Ok(_) | Err(ValidationError::NotConnected) => {
                    self.nodi[current_index] = current_node.clone();
                    self.nodi[neighbour_index] = neighbour.clone();

                    match current_node.node_type {
                        SimulationControllerNodeType::DRONE { .. } => {
                            if let Some(controller_send_current) = self.sc_drone_channels.get(&current_node.node_id) {
                                if let Some(neighbour_send) = self.packet_node_channels.get(&neighbour.node_id) {
                                    controller_send_current.send(DroneCommand::AddSender(neighbour.node_id, neighbour_send.clone().0)).expect("Error sending the command...");
                                }
                            }
                        }
                        SimulationControllerNodeType::CLIENT { .. } => {
                            if let Some(controller_send_current) = self.sc_client_channels.get(&current_node.node_id) {
                                if let Some(neighbour_send) = self.packet_node_channels.get(&neighbour.node_id) {
                                    controller_send_current.send(ClientCommand::AddSender(neighbour.node_id, neighbour_send.clone().0)).expect("Error sending the command...");
                                }
                            }
                        }
                        SimulationControllerNodeType::SERVER { .. } => {
                            if let Some(controller_send_current) = self.sc_server_channels.get(&current_node.node_id) {
                                if let Some(neighbour_send) = self.packet_node_channels.get(&neighbour.node_id) {
                                    controller_send_current.send(ServerCommand::AddSender(neighbour.node_id, neighbour_send.clone().0)).expect("Error sending the command...");
                                }
                            }
                        }
                    }

                    match neighbour.node_type {
                        SimulationControllerNodeType::DRONE { .. } => {
                            if let Some(controller_send_neighbour) = self.sc_drone_channels.get(&neighbour.node_id) {
                                if let Some(current_send) = self.packet_node_channels.get(&current_node.node_id) {
                                    controller_send_neighbour.send(DroneCommand::AddSender(current_node.node_id, current_send.clone().0)).expect("Error sending the command...");
                                }
                            }
                        }
                        SimulationControllerNodeType::CLIENT { .. } => {
                            if let Some(controller_send_neighbour) = self.sc_client_channels.get(&neighbour.node_id) {
                                if let Some(current_send) = self.packet_node_channels.get(&current_node.node_id) {
                                    controller_send_neighbour.send(ClientCommand::AddSender(current_node.node_id, current_send.clone().0)).expect("Error sending the command...");
                                }
                            }
                        }
                        SimulationControllerNodeType::SERVER { .. } => {
                            if let Some(controller_send_neighbour) = self.sc_server_channels.get(&neighbour.node_id) {
                                if let Some(current_send) = self.packet_node_channels.get(&current_node.node_id) {
                                    controller_send_neighbour.send(ServerCommand::AddSender(current_node.node_id, current_send.clone().0)).expect("Error sending the command...");
                                }
                            }
                        }
                    }
                    sleep(Duration::from_millis(100));
                    for client_command in self.sc_client_channels.clone() {
                        warn!("SC : added sender and requesting client {} to update network", client_command.0);
                        client_command.1.send(ClientCommand::RequestNetworkDiscovery).expect("Error sending Request Network Discovery");
                    }
                    for server_command in self.sc_server_channels.clone() {
                        warn!("SC : added sender and requesting server {} to update network", server_command.0);
                        server_command.1.send(ServerCommand::RequestNetworkDiscovery).expect("Error sending Request Network Discovery");
                    }
                }
                Err(_) => {
                    self.panel.central_panel.active_error = result;
                    self.panel.central_panel.popup_timer = Some(Instant::now());
                }
            }
            self.panel.reset();
        }
    }

    pub fn remove_sender(&mut self, neighbour_id: NodeId) {
        let current_node_id = self.panel.central_panel.selected_node.clone().unwrap().node_id;

        let current_node_index = self.nodi.iter().position(|node| node.node_id == current_node_id);
        let neighbour_index = self.nodi.iter().position(|node| node.node_id == neighbour_id);

        if let (Some(current_index), Some(neighbour_index)) = (current_node_index, neighbour_index) {
            let mut current_node = self.nodi[current_index].clone();
            let mut neighbour = self.nodi[neighbour_index].clone();

            current_node.neighbours.retain(|&node| node != neighbour.node_id);
            neighbour.neighbours.retain(|&node| node != current_node.node_id);

            let mut node_verification = self.nodi.clone();
            node_verification[current_index] = current_node.clone();
            node_verification[neighbour_index] = neighbour.clone();
            let result = validate_network(&node_verification);
            if result.is_ok() {
                self.nodi[current_index] = current_node.clone();
                self.nodi[neighbour_index] = neighbour.clone();

                match current_node.node_type {
                    SimulationControllerNodeType::DRONE { .. } => {
                        if let Some(controller_send_current) = self.sc_drone_channels.get(&current_node.node_id) {
                            controller_send_current.send(DroneCommand::RemoveSender(neighbour.node_id)).expect("Error sending the command...");
                        }
                    }
                    SimulationControllerNodeType::CLIENT { .. } => {
                        if let Some(controller_send_current) = self.sc_client_channels.get(&current_node.node_id) {
                            controller_send_current.send(ClientCommand::RemoveSender(neighbour.node_id)).expect("Error sending the command...");
                        }
                    }
                    SimulationControllerNodeType::SERVER { .. } => {
                        if let Some(controller_send_current) = self.sc_server_channels.get(&current_node.node_id) {
                            controller_send_current.send(ServerCommand::RemoveSender(neighbour.node_id)).expect("Error sending the command...");
                        }
                    }
                }

                match neighbour.node_type {
                    SimulationControllerNodeType::DRONE { .. } => {
                        if let Some(controller_send_neighbour) = self.sc_drone_channels.get(&neighbour.node_id) {
                            controller_send_neighbour.send(DroneCommand::RemoveSender(current_node.node_id)).expect("Error sending the command...");
                        }
                    }
                    SimulationControllerNodeType::CLIENT { .. } => {
                        if let Some(controller_send_neighbour) = self.sc_client_channels.get(&neighbour.node_id) {
                            controller_send_neighbour.send(ClientCommand::RemoveSender(current_node.node_id)).expect("Error sending the command...");
                        }
                    }
                    SimulationControllerNodeType::SERVER { .. } => {
                        if let Some(controller_send_neighbour) = self.sc_server_channels.get(&neighbour.node_id) {
                            controller_send_neighbour.send(ServerCommand::RemoveSender(current_node.node_id)).expect("Error sending the command...");
                        }
                    }
                }
                sleep(Duration::from_millis(100));
                for client_command in self.sc_client_channels.clone(){
                    warn!("SC : removed sender and requesting client {} to update network", client_command.0);
                    client_command.1.send(ClientCommand::RequestNetworkDiscovery).expect("Error sending Request Network Discovery");
                }
                for server_command in self.sc_server_channels.clone(){
                    warn!("SC : removed sender and requesting server {} to update network", server_command.0);
                    server_command.1.send(ServerCommand::RequestNetworkDiscovery).expect("Error sending Request Network Discovery");
                }
            }
            else{
                self.panel.central_panel.active_error = result;
                self.panel.central_panel.popup_timer = Some(Instant::now());
            }
            self.panel.reset();
        }
    }

    pub fn crash(&mut self){
        if let Some(current_node) = self.panel.central_panel.selected_node.clone(){
            let mut node_verification = self.nodi.clone();
            for elem in current_node.clone().neighbours {
                if let Some(node_index) = node_verification.iter().position(|node| node.node_id == elem) {
                    let mut neighbour = node_verification[node_index].clone();
                    neighbour.neighbours.retain(|node_id| node_id.clone() != current_node.node_id);
                    node_verification[node_index] = neighbour.clone();
                }
            }
            node_verification.retain(|node| node.node_id != current_node.node_id);
            let result = validate_network(&node_verification);
            if result.is_ok() {
                self.remove_sender_crashing(current_node.clone());
                self.nodi.retain(|node| node.node_id != current_node.node_id);
                if let Some(controller_send) = self.sc_drone_channels.get(&current_node.node_id) {
                    controller_send.send(DroneCommand::Crash).expect("Error sending the command...");
                }
            }
            else{
                self.panel.central_panel.active_error = result;
                self.panel.central_panel.popup_timer = Some(Instant::now());
            }
            self.panel.reset();
            self.panel.central_panel.selected_node = None;
        }
    }

    pub fn set_pdr(&mut self, set_pdr: f32){
        if let Some(mut current_node) = self.panel.central_panel.selected_node.clone() {
            let node_index = self.nodi.iter().position(|node| node.node_id == current_node.node_id);
            if let Some(index) = node_index{
                if let SimulationControllerNodeType::DRONE { pdr, .. } = &mut current_node.node_type {
                    *pdr = set_pdr;
                }
                self.nodi[index] = current_node.clone();

                if let Some(controller_send) = self.sc_drone_channels.get(&current_node.node_id) {
                    controller_send.send(DroneCommand::SetPacketDropRate(set_pdr)).expect("Error sending the command...");
                }
            }
        }
        self.panel.reset();
    }

    pub fn spawn(&mut self, pdr: f32) {
        let (packet_send, packet_recv) = unbounded();
        let (command_send, command_recv) = unbounded::<DroneCommand>();

        let mut max_id = self.nodi.iter().map(|n| n.node_id).max().expect("Vettore di nodi vuoto");
        max_id = max_id + 1;

        self.sc_drone_channels.insert(max_id, command_send.clone());
        self.packet_node_channels.insert(max_id, (packet_send, packet_recv.clone()));

        SimulationControllerNode::new(SimulationControllerNodeType::DRONE { drone_channel: command_send, pdr, drone_type: "Dronegowski".to_string() }, max_id, vec![], &mut self.nodi);
        let event_send = self.sc_drone_event_send.clone();
        self.handles.push(thread::spawn(move || {
            let mut drone = Dronegowski::new(max_id, event_send, command_recv, packet_recv, HashMap::new(), pdr);
            drone.run();
        }));
    }

    fn remove_sender_crashing(&mut self, node: SimulationControllerNode){
        for elem in node.neighbours{
            if let Some(node_index) = self.nodi.iter().position(|node| node.node_id == elem) {
                let mut neighbour = self.nodi[node_index].clone();
                neighbour.neighbours.retain(|node_id| node_id.clone() != node.node_id);
                self.nodi[node_index] = neighbour.clone();
                match neighbour.node_type{
                    SimulationControllerNodeType::SERVER { .. } => {
                        if let Some(channel) = self.sc_server_channels.get(&elem){
                            channel.send(ServerCommand::RemoveSender(node.node_id)).expect("Error sending the command");
                        }
                    }
                    SimulationControllerNodeType::CLIENT { .. } => {
                        if let Some(channel) = self.sc_client_channels.get(&elem){
                            channel.send(ClientCommand::RemoveSender(node.node_id)).expect("Error sending the command");
                        }
                    }
                    SimulationControllerNodeType::DRONE { .. } => {
                        if let Some(channel) = self.sc_drone_channels.get(&elem){
                            channel.send(DroneCommand::RemoveSender(node.node_id)).expect("Error sending the command");
                        }
                    }
                }
            }
        }
        sleep(Duration::from_millis(100));
        for client_command in self.sc_client_channels.clone(){
            warn!("SC : removed sender and requesting client {} to update network", client_command.0);
            client_command.1.send(ClientCommand::RequestNetworkDiscovery).expect("Error sending Request Network Discovery");
        }
        for server_command in self.sc_server_channels.clone(){
            warn!("SC : removed sender and requesting server {} to update network", server_command.0);
            server_command.1.send(ServerCommand::RequestNetworkDiscovery).expect("Error sending Request Network Discovery");
        }
    }
}

impl DronegowskiSimulationController<'_>{
    fn handle_drone_event(&mut self, drone_event: DroneEvent){
        match drone_event {
            DroneEvent::ControllerShortcut(packet) => {
                let node_id = packet.clone().routing_header.hops[packet.routing_header.hops.len()-1];
                let node_index = self.nodi.iter().position(|node| node.node_id == node_id);
                if let Some(idx) = node_index {
                    let mut node = self.nodi[idx].clone();
                    match node.node_type {
                        SimulationControllerNodeType::CLIENT { .. } => {
                            println!("Sto inviando un controller shortcut al client {}", node_id);
                            if let Some(controller_send_current) = self.sc_client_channels.get(&node.node_id) {
                                controller_send_current.send(ClientCommand::ControllerShortcut(packet)).expect("Error sending a controller shortcut to a client");
                            }
                        }
                        SimulationControllerNodeType::SERVER { .. } => {
                            println!("Sto inviando un controller shortcut al server {}", node_id);
                            if let Some(controller_send_current) = self.sc_server_channels.get(&node.node_id) {
                                controller_send_current.send(ServerCommand::ControllerShortcut(packet)).expect("Error sending a controller shortcut to a server");
                            }
                        }
                        _ => {}
                    }
                }

            }
            _ => {}
        }
    }
    fn handle_client_event(&mut self, client_event: ClientEvent){
        match client_event.clone() {
            ClientEvent::DebugMessage(..) => {
                self.panel.bottom_left_panel.event.push(Event::ClientEvent(client_event));
            }
            ClientEvent::Route(..) => {
                self.panel.bottom_left_panel.event.push(Event::ClientEvent(client_event));
            }
            _ => {}
        }
    }
    fn handle_server_event(&mut self, server_event: ServerEvent){
        match server_event.clone() {
            ServerEvent::DebugMessage(..) => {
                self.panel.bottom_left_panel.event.push(Event::ServerEvent(server_event));
            }
            ServerEvent::Route(..) => {
                self.panel.bottom_left_panel.event.push(Event::ServerEvent(server_event));
            }
            _ => {}
        }
    }
}