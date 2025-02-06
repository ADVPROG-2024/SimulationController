use crossbeam_channel::{unbounded, Receiver, Sender};
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerType, ServerMessages, TestMessage, ClientMessages, ClientType};
use eframe::egui;
use eframe::egui::Context;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController, SimulationControllerNode, Panel}; // Import your controller and related types
use dronegowski_utils::network::SimulationControllerNodeType;
use wg_2024::packet::Packet;

// Helper function to create a minimal SimulationControllerNode (for testing)
fn create_test_node(node_id: NodeId, node_type: SimulationControllerNodeType) -> SimulationControllerNode {
    SimulationControllerNode {
        node_type,
        node_id,
        neighbours: Vec::new(),
    }
}


struct TestApp<'a> {
    controller: DronegowskiSimulationController<'a>,
    client_cmd_sender: Sender<ClientCommand>, // To simulate GUI interactions
    client_event_receiver: Receiver<ClientEvent>, //To simulate DronegowskiClient
    handles: &'a mut Vec<JoinHandle<()>>,

}

impl<'a> eframe::App for TestApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("TestApp update called"); // Debug print
        self.controller.update(ctx, _frame); // Call the controller's update
        println!("TestApp update end"); // Debug print
    }
}


#[test]
fn test_client_server_type_request() {

    let mut handles = Vec::new(); //For the thread
    let (client_cmd_sender, client_cmd_receiver) = unbounded();
    let (client_event_sender, client_event_receiver) = unbounded();

    let client_node = create_test_node(2, SimulationControllerNodeType::CLIENT { client_type: ClientType::WebBrowsers, client_channel: client_cmd_sender.clone() });
    let server_node = create_test_node(1, SimulationControllerNodeType::SERVER { server_type: ServerType::Content, server_channel: unbounded().0 });

    let nodi = vec![client_node.clone(), server_node.clone()];

    let mut sc_client_channels = HashMap::new();
    sc_client_channels.insert(2, client_cmd_sender.clone());


    let mut controller = DronegowskiSimulationController {
        nodi,
        sc_drone_channels: HashMap::new(), // Not used in this test
        sc_client_channels,
        sc_server_channels: HashMap::new(), // Not used in this test
        sc_drone_event_send: unbounded().0,  // Not used
        sc_drone_event_recv: unbounded().1,  // Not used
        sc_client_event_recv: client_event_receiver.clone(),
        sc_server_event_recv: unbounded().1,  // Not used
        packet_node_channels: HashMap::new(),   // Not used
        handles: &mut handles,
        panel: Panel::default(),
    };

    // 1. Simulate opening the client GUI popup.
    controller.panel.central_panel.active_popups.insert(2, client_node.clone());

    // Create the TestApp
    let mut app = TestApp {
        controller,
        client_cmd_sender,
        client_event_receiver,
        handles: &mut handles,
    };

    // Use eframe::run_test_app to run the test in a headless eframe environment
    eframe::run_test_app(app).expect("Test app failed");


    // 2. Simulate selecting "ServerType" and clicking "Send".
    //    We do this by directly sending the ClientCommand.
    println!("Test: Sending ClientCommand::ServerType"); // Debug print
    app.client_cmd_sender.send(ClientCommand::ServerType(1)).unwrap();

    // 3.  Give the controller a chance to process and the client to respond
    thread::sleep(Duration::from_millis(100));


    // 4. Simulate the client sending back a ServerTypeReceived event.
    println!("Test: Waiting for ClientEvent (simulated from client)..."); // Debug print
    let received_event = app.client_event_receiver.recv().unwrap();
    println!("Test: Received ClientEvent: {:?}", received_event); // Debug print


    println!("Test: Sending ClientCommand::ServerType (again, for testing)"); // Debug print
    app.client_cmd_sender.send(ClientCommand::ServerType(1)).unwrap();

    // 3.  Give the controller a chance to process and the client to respond
    thread::sleep(Duration::from_millis(100));
    //Simulate the event from the client
    println!("Test: Sending ClientEvent::ServerTypeReceived (simulated from client)"); // Debug print
    app.controller.sc_client_event_send.send(ClientEvent::ServerTypeReceived(1, ServerType::Content)).unwrap();

    // 3.  Give the controller a chance to process and the client to respond
    thread::sleep(Duration::from_millis(100));

    // 5. Verify the GUI state was updated (check within the controller).
    let id = egui::Id::new(2).with("client_gui_state"); // ID for client 2
    // You need a way to access the egui::Context within your test. run_test_app provides one
    let ctx = &app.controller.ctx; // Get the context (you may need to make ctx public or provide an accessor)
    let server_type = ctx.data(|data| data.get_temp::<Option<(NodeId, ServerType)>>(id.with("server_type")).cloned().flatten());
    assert_eq!(server_type, Some((1, ServerType::Content)));
    ctx.request_repaint(); // VERY IMPORTANT: Request a repaint

}