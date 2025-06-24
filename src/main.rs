mod main_app;
mod resource;
use main_app::*;

fn main() {
    let native_opt = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_inner_size([800.0, 50.0])
            .with_resizable(false)
            .with_decorations(false)
            .with_position([1.0, 1020.0]),
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
