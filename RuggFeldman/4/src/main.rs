#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
use eframe::egui;
use std::time::SystemTime;


#[derive(Default)]
struct Application { status: String, length: i32, tests: i32}
impl Application {
    fn new() -> Self {
        Self { status: "Hello!".to_string(), length: 500000, tests: 10 }
    }
}

fn update_text(label_text: &mut String, context: &egui::Context, new_text: &str) {
    label_text.clear();
    label_text.push_str(new_text);    
    context.request_repaint();
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)  {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RuggFeldman Benchmark 4");
            ui.add(egui::Slider::new(&mut self.tests, 200..=999999999).text("Runs"));
            ui.add(egui::Label::new(&self.status));
            
            if ui.button("Begin test").clicked() {
                let mut time: i32 = 0;
                for _ in 1..(self.tests + 1) {
                    let start = SystemTime::now();
                    
                    println!("S");
                    let mut K = 0;
                    for K in 1..1000 {
                        let A=K/2*3+4-5;
                    }
                    println!("E");

                    let finish = SystemTime::now();
                    let duration = finish
                        .duration_since(start)
                        .unwrap()
                        .as_millis() as i32;
                    time += duration;
                }
                
                update_text(&mut self.status, &ctx, 
                format!("Average time: {} ms | Overall time: {} ms (≈{} s)\nScore: {}", 
                    time / (self.tests + 1), time, time / 1000,
                    (time ^ 2) / 6).as_str());
            }
        });
    }
}



fn main() -> Result<(), eframe::Error>  {
    const ICON: &[u8; 15952] = include_bytes!("../icon.png");
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        icon_data: Some(eframe::IconData {
            rgba: ICON.to_vec(),
            width: 32,
            height: 32,
        }),
        ..Default::default()
    };

    eframe::run_native(
        "RuggFeldman Benchmark 4", 
        options, 
        Box::new(|_cc| Box::<Application>::default())
    )    
}
