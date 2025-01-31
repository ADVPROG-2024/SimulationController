use dronegowski_utils::network::SimulationControllerNode;
use eframe::epaint::Color32;
use wg_2024::network::NodeId;

pub struct Panel{
    pub central_panel: CentralPanel,
    pub bottom_panel: BottomPanel,
    pub left_panel: LeftPanel
}


pub struct CentralPanel {
    pub selected_node: Option<SimulationControllerNode>,
}

pub struct BottomPanel {
    pub active_add_sender: bool,
}

pub struct LeftPanel {
    pub active_left_button: LeftButton,
    pub left_button: String
}

#[derive(PartialEq)]
pub enum LeftButton{
    Network,
    Notifiche,
    Lista
}
impl Panel{
    pub fn default() -> Self {
        Self{
            central_panel: CentralPanel::new(),
            bottom_panel: BottomPanel::new(),
            left_panel: LeftPanel::new()
        }
    }

    pub fn reset(&mut self){
        self.bottom_panel.reset();
    }
}

impl BottomPanel{
    fn new() -> Self{
        Self{
            active_add_sender: false,
        }
    }

    fn reset(&mut self){
        self.active_add_sender = false;
    }
}

impl LeftPanel{
    fn new() -> Self{
        Self{
            active_left_button: LeftButton::Network,
            left_button: "Visualizzazione network".to_string()
        }
    }
}

impl CentralPanel{
    fn new() -> Self{
        Self{
            selected_node: None,
        }
    }
}