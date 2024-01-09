use eframe::egui;

fn main() {
    eframe::run_native(
        "egui-phosphor demo",
        Default::default(),
        Box::new(|cc| Box::new(Demo::new(cc))),
    )
    .unwrap();
}

struct Demo;

impl Demo {
    fn new(cc: &eframe::CreationContext) -> Self {
        egui_phosphor::add_all_enabled!(cc.egui_ctx);

        Self
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for size in [16.0, 24.0, 32.0, 48.0, 64.0, 92.0] {
                let resp = ui.label(
                    egui::RichText::new(format!("FILE_CODE {}", egui_phosphor::regular::FILE_CODE))
                        .size(size),
                );
                ui.painter()
                    .debug_rect(resp.rect, egui::Color32::RED, format!("{size}"));
            }
            for size in [16.0, 92.0] {
                let _ = ui.button(
                    egui::RichText::new(format!("FILE_CODE {}", egui_phosphor::regular::FILE_CODE))
                        .size(size),
                );
            }
        });
    }
}
