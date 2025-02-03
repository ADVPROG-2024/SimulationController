use std::collections::HashMap;
use std::thread;
use std::thread::{JoinHandle};
use wg_2024;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use dronegowski::Dronegowski;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use eframe::egui;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::{NodeId, SourceRoutingHeader};
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui::Color32;
use wg_2024::drone::Drone;
use wg_2024::packet::{Fragment, Packet};
use wg_2024::packet::PacketType::MsgFragment;
use crate::sc_utils::{Panel};

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

    fn create(cc: &eframe::CreationContext<'_>,
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
            nodi,
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        loop {
            select! {
                recv(self.sc_drone_event_recv) -> drone_event_res => {
                    if let Ok(drone_event) = drone_event_res {
                        self.handle_drone_event(drone_event);
                    }
                },

                recv(self.sc_client_event_recv) -> client_event_res => {
                    if let Ok(client_event) = client_event_res {
                        self.handle_client_event(client_event);
                    }
                },

                recv(self.sc_server_event_recv) -> server_event_res => {
                    if let Ok(server_event) = server_event_res {
                        self.handle_server_event(server_event);
                    }
                }
                default => break,
            }
        }

        egui::TopBottomPanel::bottom("bottom_bar").frame(egui::Frame::none().fill(Color32::from_rgb(94, 199, 113))).resizable(false).exact_height(300.).show(ctx, |ui| {
            match &self.panel.central_panel.selected_node.clone(){
                None => { self.bottom_panel(ui); }
                Some(node) => {
                    match node.node_type {
                        SimulationControllerNodeType::DRONE {..} => {
                            self.bottom_panel_drone(ui, self.panel.central_panel.selected_node.clone().unwrap());
                        }
                        SimulationControllerNodeType::CLIENT {..} => {
                            self.bottom_panel_client(ui, self.panel.central_panel.selected_node.clone().unwrap());
                        }
                        SimulationControllerNodeType::SERVER {..} => {
                            self.bottom_panel_server(ui, self.panel.central_panel.selected_node.clone().unwrap());
                        }
                    }
                }
            }
        });

        egui::SidePanel::left("side_panel").resizable(false).exact_width(300.).show(ctx, |ui| {
            self.left_side_panel(ui);
        });

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            self.central_panel(ui);
        });

        let mut popups_to_remove = vec![];
        for (node_id, node) in &self.panel.central_panel.active_popups {
            let mut selected_option = None;

            egui::Window::new(format!("Client: {}", node_id))
                .collapsible(false)
                .resizable(true)
                .frame(egui::Frame::window(&ctx.style()).fill(Color32::WHITE)) // Sfondo bianco
                .show(ctx, |ui| {
                    // Bottone per chiudere la finestra (in alto a destra)
                    ui.horizontal(|ui| {
                        ui.label(format!("Dettagli del nodo {}", node_id));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.add(egui::Button::new("X").fill(Color32::RED)).clicked() {
                                popups_to_remove.push(*node_id);
                            }
                        });
                    });

                    ui.separator();

                    // Menu a tendina
                    ui.label("Scegli un'azione:");
                    egui::ComboBox::from_label("Opzioni")
                        .selected_text(selected_option.unwrap_or("Seleziona un'opzione"))
                        .show_ui(ui, |ui| {
                            for option in &[
                                "ServerType",
                                "FileList",
                                "File",
                                "Media",
                                "ClientList",
                                "RegistrationToChat",
                                "MessageFor",
                            ] {
                                if ui.selectable_value(&mut selected_option, Option::from(*option), *option).clicked() {
                                    selected_option = Some(*option);
                                }
                            }
                        });

                    // Pulsante "Invia"
                    if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("Invia")).clicked() {
                        // Logica per il pulsante invia
                        println!("Opzione selezionata: {:?}", selected_option);
                    }
                });
        }

        // Rimuovi i popup chiusi
        for node_id in popups_to_remove {
            self.panel.central_panel.active_popups.remove(&node_id);
        }
    }
}

impl DronegowskiSimulationController<'_>{
    pub fn add_sender(&mut self, neighbour_id: NodeId){
        let current_node_id = self.panel.central_panel.selected_node.clone().unwrap().node_id;

        // Trova gli indici dei nodi invece di tenerne il riferimento diretto
        let current_node_index = self.nodi.iter().position(|node| node.node_id == current_node_id);
        let neighbour_index = self.nodi.iter().position(|node| node.node_id == neighbour_id);

        if let (Some(current_index), Some(neighbour_index)) = (current_node_index, neighbour_index) {
            // Cloniamo i nodi per poterli modificare
            let mut current_node = self.nodi[current_index].clone();
            let mut neighbour = self.nodi[neighbour_index].clone();

            // Aggiorniamo le liste di vicini
            current_node.neighbours.push(neighbour_id);
            neighbour.neighbours.push(current_node_id);

            // Aggiorniamo la lista rimuovendo i vecchi nodi e aggiungendo quelli aggiornati
            self.nodi[current_index] = current_node.clone();
            self.nodi[neighbour_index] = neighbour.clone();

            match current_node.node_type {
                SimulationControllerNodeType::DRONE {..} => {
                    if let Some(controller_send_current) = self.sc_drone_channels.get(&current_node.node_id){
                        if let Some(neighbour_send) = self.packet_node_channels.get(&neighbour.node_id) {
                            controller_send_current.send(DroneCommand::AddSender(neighbour.node_id, neighbour_send.clone().0)).expect("Error sending the command...");
                        }
                    }
                }
                SimulationControllerNodeType::CLIENT {..} => {
                    if let Some(controller_send_current) = self.sc_client_channels.get(&current_node.node_id){
                        if let Some(neighbour_send) = self.packet_node_channels.get(&neighbour.node_id){
                            controller_send_current.send(ClientCommand::AddSender(neighbour.node_id, neighbour_send.clone().0)).expect("Error sending the command...");
                        }
                    }
                }
                SimulationControllerNodeType::SERVER {..} => {
                    if let Some(controller_send_current) = self.sc_server_channels.get(&current_node.node_id){
                        if let Some(neighbour_send) = self.packet_node_channels.get(&neighbour.node_id){
                            //controller_send_current.send(ServerCommand::AddSender(neighbour.node_id, neighbour_send.clone().0)).expect("Error sending the command...");
                        }
                    }
                }
            }

            match neighbour.node_type {
                SimulationControllerNodeType::DRONE {..} => {
                    if let Some(controller_send_neighbour) = self.sc_drone_channels.get(&neighbour.node_id){
                        if let Some(current_send) = self.packet_node_channels.get(&neighbour.node_id){
                            controller_send_neighbour.send(DroneCommand::AddSender(current_node.node_id, current_send.clone().0)).expect("Error sending the command...");
                        }
                    }
                }
                SimulationControllerNodeType::CLIENT {..} => {
                    if let Some(controller_send_neighbour) = self.sc_client_channels.get(&neighbour.node_id){
                        if let Some(current_send) = self.packet_node_channels.get(&neighbour.node_id){
                            controller_send_neighbour.send(ClientCommand::AddSender(current_node.node_id, current_send.clone().0)).expect("Error sending the command...");
                        }
                    }
                }
                SimulationControllerNodeType::SERVER {..} => {
                    if let Some(controller_send_neighbour) = self.sc_server_channels.get(&neighbour.node_id){
                        if let Some(current_send) = self.packet_node_channels.get(&neighbour.node_id){
                            //controller_send_neighbour.send(ServerCommand::AddSender(current_node.node_id, current_send.clone().0)).expect("Error sending the command...");
                        }
                    }
                }
            }
        }
        self.panel.reset();
    }

    pub fn remove_sender(&mut self, neighbour_id: NodeId) {
        let current_node_id = self.panel.central_panel.selected_node.clone().unwrap().node_id;

        // Trova gli indici dei nodi invece di tenerne il riferimento diretto
        let current_node_index = self.nodi.iter().position(|node| node.node_id == current_node_id);
        let neighbour_index = self.nodi.iter().position(|node| node.node_id == neighbour_id);

        if let (Some(current_index), Some(neighbour_index)) = (current_node_index, neighbour_index) {
            let mut current_node = self.nodi[current_index].clone();
            let mut neighbour = self.nodi[neighbour_index].clone();

            // Aggiorniamo le liste di vicini
            current_node.neighbours.retain(|&node| node != neighbour.node_id);
            neighbour.neighbours.retain(|&node| node != current_node.node_id);

            //println!("Vicini nodo {}: {:?}", current_node.node_id,current_node.neighbours);
            //println!("Vicini nodo {}: {:?}", neighbour.node_id,neighbour.neighbours);

            // Aggiorniamo la lista rimuovendo i vecchi nodi e aggiungendo quelli aggiornati
            self.nodi[current_index] = current_node.clone();
            self.nodi[neighbour_index] = neighbour.clone();

            match current_node.node_type {
                SimulationControllerNodeType::DRONE {..} => {
                    if let Some(controller_send_current) = self.sc_drone_channels.get(&current_node.node_id){
                        controller_send_current.send(DroneCommand::RemoveSender(neighbour.node_id)).expect("Error sending the command...");
                    }
                }
                SimulationControllerNodeType::CLIENT {..} => {
                    if let Some(controller_send_current) = self.sc_client_channels.get(&current_node.node_id){
                        controller_send_current.send(ClientCommand::RemoveSender(neighbour.node_id)).expect("Error sending the command...");
                    }
                }
                SimulationControllerNodeType::SERVER {..} => {
                    if let Some(controller_send_current) = self.sc_server_channels.get(&current_node.node_id){
                        //controller_send_current.send(ServerCommand::RemoveSender(neighbour.node_id)).expect("Error sending the command...");
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
                        //controller_send_neighbour.send(ServerCommand::RemoveSender(current_node.node_id)).expect("Error sending the command...");
                    }
                }
            }
        }
        self.panel.reset();
    }

    pub fn crash(&mut self){
        if let Some(current_node) = self.panel.central_panel.selected_node.clone(){
            self.nodi.retain(|node| node.node_id != current_node.node_id);
            if let Some(controller_send) = self.sc_drone_channels.get(&current_node.node_id) {
                controller_send.send(DroneCommand::Crash).expect("Error sending the command...");
            }
        }
        self.panel.reset();
    }

    pub fn set_pdr(&mut self, set_pdr: f32){
        if let Some(mut current_node) = self.panel.central_panel.selected_node.clone() {
            self.nodi.retain(|node| node.node_id != current_node.node_id);

            if let SimulationControllerNodeType::DRONE { pdr, .. } = &mut current_node.node_type {
                *pdr = set_pdr;
            }

            self.nodi.push(current_node.clone());

            if let Some(controller_send) = self.sc_drone_channels.get(&current_node.node_id) {
                controller_send.send(DroneCommand::SetPacketDropRate(set_pdr)).expect("Error sending the command...");
            }
        }
        self.panel.reset();
    }

    pub fn spawn(&mut self, pdr: f32){
        let (packet_send, packet_recv) = unbounded();
        let (command_send, command_recv) = unbounded::<DroneCommand>();
        let (event_send, event_recv) = unbounded::<DroneEvent>();

        let mut max_id= self.nodi.iter().map(|n| n.node_id).max().expect("Vettore di nodi vuoto");
        max_id = max_id + 1;

        self.sc_drone_channels.insert(max_id, command_send.clone());
        self.packet_node_channels.insert(max_id,(packet_send, packet_recv.clone()));

        SimulationControllerNode::new(SimulationControllerNodeType::DRONE { drone_channel: command_send, pdr }, max_id, vec![], &mut self.nodi);

        self.handles.push(thread::spawn(move || {
            let mut drone = Dronegowski::new(max_id, event_send, command_recv, packet_recv, HashMap::new(), pdr);

            drone.run();
        }));
    }
}

impl DronegowskiSimulationController<'_>{
    fn handle_drone_event(&mut self, drone_event: DroneEvent){
        println!("Qualcosa drone");
    }

    fn handle_client_event(&mut self, client_event: ClientEvent){
        println!("Qualcosa client");
        println!("{:?}", client_event);

    }

    fn handle_server_event(&mut self, server_event: ServerEvent){
        println!("Qualcosa server");

    }

    pub fn send_packet_test(&mut self){
        let fragment1 = Packet { routing_header: SourceRoutingHeader { hop_index: 0, hops: vec![1, 2, 3] }, session_id: 42, pack_type: MsgFragment(Fragment { fragment_index: 1, total_n_fragments: 2, length: 43, data: [0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 81, 117, 101, 115, 116, 111, 32, 195, 168, 32, 117, 110, 32, 109, 101, 115, 115, 97, 103, 103, 105, 111, 32, 100, 105, 32, 116, 101, 115, 116, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }) };

        let fragment2 = Packet {
            pack_type: MsgFragment(Fragment {
                fragment_index: 0,
                total_n_fragments: 2,
                length: 12,
                data: [2, 0, 0, 0, 234, 0, 0, 0, 0, 0, 0, 0, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3, 4, 5, 6, 7, 1, 3],
            }),
            routing_header: SourceRoutingHeader {
                hop_index: 0,
                hops: vec![1, 2],
            },
            session_id: 42,
        };

        let channel_send1 = self.packet_node_channels.get(&4);
        let channel_send2 = self.packet_node_channels.get(&5);

        if let Some(send_channel) = channel_send1{
            send_channel.0.send(fragment1.clone()).unwrap();
            send_channel.0.send(fragment2.clone()).unwrap();
        }

        if let Some(send_channel) = channel_send2{
            send_channel.0.send(fragment1).unwrap();
            send_channel.0.send(fragment2).unwrap();
        }
    }
}