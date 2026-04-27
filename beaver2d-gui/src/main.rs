// Beaver2D - A 2D vector graphics editor written in Rust using eframe and egui.
// Copyright (C) 2026  Beaver2D Team
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License

use eframe::egui;
use egui::menu;
use egui_extras;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Beaver2D",
        native_options,
        Box::new(|cc| Ok(Box::new(Beaver2D::new(cc)))),
    );
}

#[derive(Default)]
struct Beaver2D {}

impl Beaver2D {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Beaver2D {
   fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    //egui_extras::install_image_loaders(ctx);

    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        show_menu(ui);
    });
    egui::SidePanel::left("Toolbox").resizable(true).show(ctx, |ui| {
           show_toolbox(ui);
       });
   }
   
}

fn show_menu(ui: &mut egui::Ui) {
    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() {
                todo!()
            }
            if ui.button("Open").clicked() {
                unimplemented!()
            }
        });
        ui.menu_button("Edit", |ui| {
            if ui.button("Undo").clicked() {
                unimplemented!()
            }
            if ui.button("Redo").clicked() {
                unimplemented!()
            }
            ui.separator();
            if ui.button("Cut").clicked() {
                unimplemented!()
            }
            if ui.button("Copy").clicked() {
                unimplemented!()
            }
            if ui.button("Paste").clicked() {
                unimplemented!()
            }
        });
        ui.menu_button("View", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
        ui.menu_button("Canvas", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
        ui.menu_button("Toolbox", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
        ui.menu_button("Layer", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
        ui.menu_button("Plug-Ins", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
        ui.menu_button("Window", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                // …
            }
        });
    });
    
}

fn show_toolbox(ui: &mut egui::Ui) {
    let img = egui::include_image!("../assets/tool_normal_icon.png");
    let img = egui::Image::new(img).max_size(egui::Vec2::new(16.0, 16.0));
    if ui.add(egui::Button::image(img)).clicked() {
        unimplemented!()
    }
}