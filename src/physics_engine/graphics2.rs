use eframe::egui::*;
// use egui::*;

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        maximized: true,
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    value: i32,
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            value: 0,
            name: "hi".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hi");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.value, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.value += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.value));
            Painter::circle(
                ui.painter(),
                Pos2::new(500.0, 200.0),
                50.0,
                Color32::from_rgb(255, 0, 0),
                Stroke::default(),
            )
        });
    }
}
