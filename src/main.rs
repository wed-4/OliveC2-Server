#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_extras::{TableBuilder, Column};
use std::vec;
use std::process;
use chrono::Local;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BailC2 ver.0.0.0",
        options,
        Box::new(|cc| {
            // This gives us image support:


            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    show_settings: bool,
    portnum: i32,
    ver_popup: bool,
}

fn timegetter() -> String {
    let now = Local::now(); // 現在時刻を取得（ローカル時間）
    return now.format("%Y/%m/%d %H:%M:%S").to_string();
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let bot = vec![
            (1, "testuser", "testhost", "windows", "1.0.1", "123.345.678.100", "192.168.11.1", "JP", 10, "ad13562785327482953", 8, "RADEON graphic", "CPU Hyperbysor", 5.85),
        ];
        let log = vec![("SUCCESS", "System Running!")];

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ctx.request_repaint();
            egui::menu::bar(ui, |ui| {
                ui.menu_button("System", |ui| {
                    if ui.button("Setting").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("Terminate").clicked() {
                        process::exit(0);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Version").clicked() {
                        rfd::MessageDialog::new()
                            .set_title("About BailC2")
                            .set_description("BailC2 v1.0.0\n© 2025 Sakuya")
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                    }
                });
            });
            ui.label(timegetter());
        });
        egui::CentralPanel::default().show(ctx, |ui| {

            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .auto_shrink([true; 2])
                .columns(Column::auto(), 14)
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.label("ID"); });
                    header.col(|ui| { ui.label("username"); });
                    header.col(|ui| { ui.label("hostname"); });
                    header.col(|ui| { ui.label("osname"); });
                    header.col(|ui| { ui.label("osversion"); });
                    header.col(|ui| { ui.label("WAN"); });
                    header.col(|ui| { ui.label("LAN"); });
                    header.col(|ui| { ui.label("Nation"); });
                    header.col(|ui| { ui.label("PID"); });
                    header.col(|ui| { ui.label("HWID"); });
                    header.col(|ui| { ui.label("cores"); });
                    header.col(|ui| { ui.label("GPUName"); });
                    header.col(|ui| { ui.label("CPUName"); });
                    header.col(|ui| { ui.label("Memory"); });
                })
                .body(|mut body| {
                    for (id, username, hostname, osname, osversion, WAN, LAN, Nation, PID, HWID, cores, GPUName, CPUName, Memory) in &bot {
                        body.row(20.0, |mut row| {
                            row.col(|ui| { ui.label(id.to_string()); });
                            row.col(|ui| { ui.label(*username); });
                            row.col(|ui| { ui.label(*hostname); });
                            row.col(|ui| { ui.label(*osname); });
                            row.col(|ui| { ui.label(*osversion); });
                            row.col(|ui| { ui.label(*WAN); });
                            row.col(|ui| { ui.label(*LAN); });
                            row.col(|ui| { ui.label(*Nation); });
                            row.col(|ui| { ui.label(PID.to_string()); });
                            row.col(|ui| { ui.label(*HWID); });
                            row.col(|ui| { ui.label(cores.to_string()); });
                            row.col(|ui| { ui.label(*GPUName); });
                            row.col(|ui| { ui.label(*CPUName); });
                            row.col(|ui| { ui.label(Memory.to_string()); });
                        });
                    }
                });

        });
        if self.show_settings {
            egui::Window::new("Settings").collapsible(true).resizable(false).movable(true).show(ctx, |ui| {
                ui.label("Server Settings");
                ui.label("port");
                ui.add(egui::DragValue::new(&mut self.portnum));


                if ui.button("close").clicked() {
                    self.show_settings = false;
                }
            });
        }

        if self.ver_popup {

        }
    }
}