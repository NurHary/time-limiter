enum TimeMode {
    Play,
    Pause,
    Stop,
}

enum Model {
    Pomodoro,
    Stopwatch,
    Timer,
}

struct SlotModel {
    model: Model,
    duration: u32,
}

impl SlotModel {
    fn new(model: Model, duration: u32) -> Self {
        Self { model, duration }
    }
}

impl Default for SlotModel {
    fn default() -> Self {
        Self {
            model: Model::Pomodoro,
            duration: 1500,
        }
    }
}

struct TimeSlot {
    mode: TimeMode,
    model: SlotModel,
}

impl Default for TimeSlot {
    fn default() -> Self {
        Self {
            mode: TimeMode::Stop,
            model: SlotModel::default(),
        }
    }
}

struct MyApp {
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
            ui.heading("Halo Dunia");
            ui.heading(format!("Waktu anda {}", self.slot[0].model.duration));
        });
    }
}

fn main() {
    let native_opt = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_inner_size([350.0, 100.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Halo Dunia",
        native_opt,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    );
}
