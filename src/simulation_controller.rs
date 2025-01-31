use std::collections::HashMap;
use wg_2024;
use crossbeam_channel::{Receiver, Sender};
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use eframe::egui;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui::accesskit::Node;
use eframe::egui::Color32;
use eframe::egui::UiKind::BottomPanel;
use crate::sc_utils::{Panel};

pub struct DronegowskiSimulationController {
    pub nodi: Vec<SimulationControllerNode>,
    pub sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
    pub sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
    pub sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
    pub sc_drone_event_recv: Receiver<DroneEvent>,
    pub sc_client_event_recv: Receiver<ClientEvent>,
    pub sc_server_event_recv: Receiver<ServerEvent>,
    pub active_popups: HashMap<NodeId, SimulationControllerNode>, // Popup attivi
    pub panel: Panel,
}

impl DronegowskiSimulationController {
    pub fn new(nodi: Vec<SimulationControllerNode>,
               sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
               sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
               sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
               sc_drone_event_recv: Receiver<DroneEvent>,
               sc_client_event_recv: Receiver<ClientEvent>,
               sc_server_event_recv: Receiver<ServerEvent>
    ){

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_fullscreen(true),
            ..Default::default()
        };

        eframe::run_native(
            "Simulation Controller",
            native_options,
            Box::new(|cc| Ok(Box::new(DronegowskiSimulationController::create(cc, nodi, sc_drone_channels, sc_client_channels, sc_server_channels, sc_drone_event_recv, sc_client_event_recv, sc_server_event_recv)))),
        ).expect("Error to run the Simulation Controller");
    }

    fn create(cc: &eframe::CreationContext<'_>,
              nodi: Vec<SimulationControllerNode>,
              sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
              sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
              sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
              sc_drone_event_recv: Receiver<DroneEvent>,
              sc_client_event_recv: Receiver<ClientEvent>,
              sc_server_event_recv: Receiver<ServerEvent>
    ) -> Self {
        Self {
            nodi,
            sc_drone_channels,
            sc_client_channels,
            sc_server_channels,
            sc_drone_event_recv,
            sc_client_event_recv,
            sc_server_event_recv,
            active_popups: HashMap::new(), // Inizializzazione vuota
            panel: Panel::default(),
        }
    }

    pub fn open_client_popup(&mut self, node: &SimulationControllerNode) {
        // Aggiungi un popup per il nodo specifico
        self.active_popups.insert(node.node_id, node.clone());
    }

    pub fn add_sender(&mut self, neighbour: NodeId){
        let mut node = self.panel.central_panel.selected_node.clone().unwrap();
        node.neighbours.push(neighbour);
        self.nodi.push(node.clone());

        match node.node_type {
            SimulationControllerNodeType::DRONE {..} => {
                if let Some(controller_send) = self.sc_drone_channels.get(&node.node_id){
                    //controller_send.send(DroneCommand::AddSender())

                }
            }
            SimulationControllerNodeType::CLIENT {..} => {
            }
            SimulationControllerNodeType::SERVER {..} => {
            }
        }
        self.panel.bottom_panel.active_add_sender = false;

        //controller_send.send(DroneCommand::AddSender(2, neighbor_send)).expect("Error sending the command...");
    }
}

impl eframe::App for DronegowskiSimulationController {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

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
        for (node_id, node) in &self.active_popups {
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
            self.active_popups.remove(&node_id);
        }
    }
}