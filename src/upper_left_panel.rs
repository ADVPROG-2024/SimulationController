use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui;
use eframe::egui::{Align, Color32, Direction, Layout, RichText, TextEdit};
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController<'_> {
    pub fn upper_left_panel_default(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new("SIMULATION COMMANDS").size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(50.);

        ui.horizontal(|ui| {
            ui.add_space(5.);
            let response = ui.add(
                TextEdit::singleline(&mut self.panel.upper_left_panel.spawn_pdr)
                    .min_size(egui::Vec2::new(200.0, 40.0))
                    .font(egui::FontId::new(30.0, egui::FontFamily::Proportional))
                    .frame(true)
                    .background_color(Color32::WHITE),
            );

            if self.panel.upper_left_panel.spawn_pdr.is_empty() && !response.has_focus() {
                ui.painter().text(
                    response.rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Set PDR",
                    egui::FontId::new(30.0, egui::FontFamily::Proportional),
                    ui.style().visuals.text_color().gamma_multiply(0.8),
                );
            }
        });

        ui.add_space(10.);
        ui.horizontal(|ui|{
            ui.add_space(232.);
            if ui.add(egui::Button::new(

                RichText::new("Spawn").color(Color32::WHITE)
                    .size(15.0)).fill(Color32::from_rgb(0, 119, 182)).min_size(egui::Vec2::new(60.0, 30.0))).clicked() && self.panel.upper_left_panel.spawn_pdr != "" {
                let mut pdr = self.panel.upper_left_panel.spawn_pdr.clone().parse::<f32>();
                if pdr.is_ok() {
                    if pdr.clone().unwrap() <= 1. && pdr.clone().unwrap() >= 0. {
                        self.spawn(pdr.clone().unwrap());
                        self.panel.upper_left_panel.spawn_pdr = "".to_string();
                    }
                }
            }
        });
    }

    pub fn upper_left_panel_drone(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.heading(RichText::new(format!("DRONE {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            
            if let SimulationControllerNodeType::DRONE { ref drone_type, .. } = node.node_type {
                ui.label(
                    RichText::new(format!("{}", drone_type))
                        .size(20.0) 
                        .color(Color32::DARK_GRAY) 
                );
            }
        });
        ui.add_space(40.);

        ui.horizontal(|ui| {
            ui.add_space(5.);
            let response = ui.add(
                TextEdit::singleline(&mut self.panel.upper_left_panel.change_pdr)
                    .min_size(egui::Vec2::new(200.0, 40.0)) 
                    .font(egui::FontId::new(30.0, egui::FontFamily::Proportional)) 
                    .frame(true) 
                    .background_color(Color32::WHITE), 
            );
            
            if self.panel.upper_left_panel.change_pdr.is_empty() && !response.has_focus() {
                ui.painter().text(
                    response.rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Set PDR",
                    egui::FontId::new(30.0, egui::FontFamily::Proportional), 
                    ui.style().visuals.text_color().gamma_multiply(0.8), 
                );
            }
        });

        ui.add_space(10.);
        ui.horizontal(|ui|{
            ui.add_space(232.);
            if ui.add(egui::Button::new(

                RichText::new("Change").color(Color32::WHITE)
                    .size(15.0)).fill(Color32::from_rgb(0, 119, 182)).min_size(egui::Vec2::new(60.0, 30.0))).clicked() && self.panel.upper_left_panel.change_pdr != "" {
                let mut pdr = self.panel.upper_left_panel.change_pdr.clone().parse::<f32>();
                if pdr.is_ok() {
                    if pdr.clone().unwrap() <= 1. && pdr.clone().unwrap() >= 0. {
                        self.set_pdr(pdr.clone().unwrap());
                        self.panel.upper_left_panel.change_pdr = "".to_string();
                    }
                }
            }
        });
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.add_space(20.);
            if ui.add(egui::Button::new(
                RichText::new("Add Sender")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::GREEN).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.add_sender = true;
            }

            ui.add_space(20.);

            if ui.add(egui::Button::new(
                RichText::new("Remove Sender")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::RED).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.remove_sender = true;
            }
            ui.add_space(10.);
        });
        ui.add_space(30.);
        ui.horizontal(|ui|{
            ui.add_space(100.);
            if ui.add(egui::Button::new(
                RichText::new("Crash")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::DARK_RED).min_size(egui::Vec2::new(90.0, 45.0))).clicked(){
                self.panel.upper_left_panel.crash = true;
            }
        });
    }

    pub fn upper_left_panel_client(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new(format!("CLIENT {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(50.);

        ui.horizontal(|ui|{
            ui.add_space(20.);
            if ui.add(egui::Button::new(
                RichText::new("Add Sender")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::GREEN).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.add_sender = true;
            }
            ui.add_space(20.);

            if ui.add(egui::Button::new(
                RichText::new("Remove Sender")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::RED).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.remove_sender = true;
            }
            ui.add_space(10.);
        });
    }

    pub fn upper_left_panel_server(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new(format!("SERVER {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(50.);

        ui.horizontal(|ui|{
            ui.add_space(20.);
            if ui.add(egui::Button::new(
                RichText::new("Add Sender")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::GREEN).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.add_sender = true;
            }

            ui.add_space(20.);

            if ui.add(egui::Button::new(
                RichText::new("Remove Sender")
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::RED).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.remove_sender = true;
            }
            ui.add_space(10.);
        });
    }
}