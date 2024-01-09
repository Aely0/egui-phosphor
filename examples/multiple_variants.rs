use eframe::{
    egui::{self, vec2, ViewportBuilder},
    run_native, NativeOptions,
};
use egui_phosphor::{bold, fill, light, regular, thin};

fn main() {
    run_native(
        "egui-phosphor demo",
        NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size(vec2(320.0, 755.0)),
            ..Default::default()
        },
        Box::new(|cc| Box::new(Demo::new(cc))),
    )
    .unwrap();
}

struct Demo;

impl Demo {
    fn new(cc: &eframe::CreationContext) -> Self {
        egui_phosphor::add_variants!(cc.egui_ctx, [thin, light, regular, bold, fill]);

        Self
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (family, icon) in [
                ("phosphor-thin", thin::FILE_CODE),
                ("phosphor-light", light::FILE_CODE),
                ("phosphor-regular", regular::FILE_CODE),
                ("phosphor-bold", bold::FILE_CODE),
                ("phosphor-fill", fill::FILE_CODE),
            ] {
                ui.heading(family);
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    for size in [16.0, 32.0, 48.0] {
                        let demo_text = format!("FILE_CODE {icon}");
                        ui.label(
                            egui::RichText::new(&demo_text)
                                .family(egui::FontFamily::Name(family.into()))
                                .size(size),
                        );
                    }
                });
            }
        });
    }
}
