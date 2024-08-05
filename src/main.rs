use egui::{
    ViewportBuilder, Context, CentralPanel, Image, ScrollArea
};


fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([840.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Show",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // gives us image support
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {}

// Two ways:
// ui.image(egui::include_image!("../assets/ferris.svg"));
// or
// ui.add(
//     Image::new(egui::include_image!("../assets/ferris.svg"))
// );

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.image(egui::include_image!("../assets/ferris.gif"));
                ui.add(
                    Image::new("https://picsum.photos/seed/1.759706314/1024")
                    .fit_to_original_size(0.15).rounding(20.0),
                );
                ui.horizontal(|ui| {
                    ui.add(
                        Image::new(egui::include_image!("../assets/ferris.svg")).max_width(200.0).rounding(10.0)
                    );
                    ui.add(
                        Image::new(egui::include_image!("../assets/ferris200.jpg")).fit_to_original_size(1.0)
                    );
                    ui.add(
                        Image::new(egui::include_image!("../assets/me.png")).fit_to_original_size(0.5)
                    );
                    ui.add(
                        Image::new(egui::include_image!("../assets/frog.png")).fit_to_original_size(0.25)
                    );
                });
            });
        });
    }
}
