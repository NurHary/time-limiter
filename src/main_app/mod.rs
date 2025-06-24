pub mod timeslot;
pub mod timeslotbutton;

pub use timeslot::*;
use timeslotbutton::*;

use ::std::io::BufReader;

use rodio::Sink;
use std::fs::File;
use std::sync::{Arc, LazyLock, Mutex};

static GLOBAL_PLAYER: LazyLock<Arc<Mutex<Option<Sink>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub struct MyApp {
    slot: Vec<TimeSlot>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            slot: vec![TimeSlot::default(), TimeSlot::default()],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.columns(3, |col| {
                        col[0].with_layout(
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.heading("Halo Dunia");
                            },
                        );
                        col[1].with_layout(
                            egui::Layout::from_main_dir_and_cross_align(
                                egui::Direction::RightToLeft,
                                egui::Align::Center,
                            ),
                            |ui| {
                                if ui.button("jajal").clicked() {
                                    println!("Bokep")
                                };
                                ui.add_space(40.);
                            },
                        );
                        col[2].with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.heading("Bajigur");
                            },
                        );
                    });
                    // This iS The Place to put your UI
                    // Main UI ing
                    // Music [] - Label: Music Name, - Timeslot [] - Slider - Label: Time - Toggle Notif []
                },
            );
        });
    }
}

impl MyApp {
    // Time Area
    fn toggle_time_play() {}
    fn toggle_notification() {}
    fn select_notification_sound() {}

    // Music Area
    fn play_music() {}
    fn stop_music() {}
    fn open_folder() {}
    fn toggle_shuffle() {}
}
