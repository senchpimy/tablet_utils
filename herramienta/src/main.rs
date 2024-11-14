#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use hyprland::data::*;

use hyprland::keyword::Keyword;
use hyprland::prelude::*;

fn main() -> eframe::Result {
    let cursor_pos = CursorPosition::get();
    let pos = cursor_pos.unwrap();

    let pos = egui::pos2(pos.x as f32, pos.y as f32); // if sometime hyprland supports this

    //hyprctl keyword windowrule "move 0 0, title:^(tablet_utils)$"
    let str = format!("move {} {}, title:^(tablet_utils)$", pos.x, pos.y);
    Keyword::set("windowrule", str).unwrap();
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([200.0, 150.0])
        .with_position(pos);
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "tablet_utils",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}

struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let btn = button_gen("Screenshot");
            if ui.add_sized([ui.available_width(), 20.], btn).clicked() {
                execute_command("flameshot gui");
            }
            ui.separator();
            let btn = button_gen("Notes");
            if ui.add_sized([ui.available_width(), 20.], btn).clicked() {
                execute_command("obsidian-cli create Untitled --open");
            }
            ui.separator();
            let btn = button_gen("Draw");
            if ui.add_sized([ui.available_width(), 20.], btn).clicked() {
                execute_command("xournalpp");
            }
            ui.separator();
        });
    }
}

fn button_gen(str: &str) -> egui::Button {
    let btn = egui::Button::new(str).frame(false);
    btn
}

fn execute_command(str: &str) {
    let mut vec: Vec<&str> = str.split(" ").collect();
    let mut output = std::process::Command::new(vec.remove(0));
    for arg in vec {
        output.arg(arg);
    }
    output.spawn().expect("Failed to execute command");
    std::process::exit(0);
}
