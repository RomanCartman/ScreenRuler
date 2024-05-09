#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::fmt::format;
use eframe::egui;
use eframe::egui::{Margin, Pos2, Rect, Widget};
use crate::egui::IconData;



fn main() {
    let options = eframe::NativeOptions {
        centered:true,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 350.0])
            .with_min_inner_size([10.0, 10.0])
            .with_transparent(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native("Screen Ruler", options, Box::new(|cc| Box::new(ScreenRulerApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct ScreenRulerApp {}

impl ScreenRulerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ScreenRulerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let my_frame = egui::containers::Frame {
            rounding: egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
            shadow: egui::epaint::Shadow::NONE,
            fill: egui::Color32::TRANSPARENT,
            stroke: egui::epaint::Stroke::NONE,
            outer_margin: Margin::same(0.0),
            inner_margin: Margin::same(0.0),

        };
        let my_frame = my_frame.multiply_with_opacity(0.05);

        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {

            ui.centered_and_justified(|ui|{
                if ui.link(format!("{}x{}", ui.available_width(),ui.available_height())).clicked(){

                }
            })
        });
    }
}
