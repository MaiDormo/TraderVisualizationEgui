mod balance;
mod app;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};
use eframe::egui::{Vec2};
use eframe::{egui, HardwareAcceleration, Theme};
use eframe::egui::plot::{PlotPoint};
use tracing::{error, info, warn};
use crate::app::App;
use crate::balance::Currency;


fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let app = App::new();

    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(950.0 as f32, 750.0 as f32)),
        min_window_size: Option::from(Vec2::new(950.0,750.0)),
        max_window_size: None,
        resizable: false,
        transparent: false,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        run_and_return: false,
        event_loop_builder: None,
        shader_version: None,
        centered: false,
    };

    let path = "./src/values.txt".to_string();
    let monitor_ref = app.balance_measurements.clone();
    let mut current_day = 0;

    thread::spawn(move || {
        //itero strategia for etc...
        //dentro il for si aggiunge valuesss
        let file = File::open(path).expect("file not found");
        let lines_iter = BufReader::new(file).lines();
        for line in lines_iter {
            match line {
                Ok(s) => {
                    let parts = s.split(' ').collect::<Vec<&str>>();
                    if parts.len() != 4 {
                        warn!("Need exactly 4 parts: {}", s);
                        continue;
                    }

                    let usd_str = parts.get(0).unwrap();
                    let eur_str = parts.get(1).unwrap();
                    let yen_str = parts.get(2).unwrap();
                    let yuan_str = parts.get(3).unwrap();


                    let usd = match usd_str.parse::<f64>() {
                        Ok(value) => value,
                        _ => {
                            warn!("failed to parse {}", usd_str);
                            continue;
                        }
                    };
                    info!("usd value gotten");

                    let eur = match eur_str.parse::<f64>() {
                        Ok(value) => value,
                        _ => {
                            warn!("failed to parse {}", eur_str);
                            continue;
                        }
                    };

                    let yen = match yen_str.parse::<f64>() {
                        Ok(value) => value,
                        _ => {
                            warn!("failed to parse {}", yen_str);
                            continue;
                        }
                    };

                    let yuan = match yuan_str.parse::<f64>() {
                        Ok(value) => value,
                        _ => {
                            warn!("failed to parse {}", yuan_str);
                            continue;
                        }
                    };

                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day,usd), Currency::USD);
                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day,eur), Currency::EUR);
                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day,yen), Currency::YEN);
                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day,yuan), Currency::YUAN);

                    //update current day
                    current_day+=1;

                    //sleep the thread so it can happen in a more visual manner
                    thread::sleep(time::Duration::from_secs(1));

                },
                _ => {
                    error!("Failed to read the line");
                    break;
                }
            }
        }
    });

    info!("Main thread started");
    eframe::run_native(
        " Trader ",
        options,
        Box::new(|_cc| Box::new(app)),
    )

}


#[derive(PartialEq, Eq)]
enum Panel {
    Merged,
    Divided
}

impl Default for Panel {
    fn default() -> Self {
        Self::Merged
    }
}



impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           //code for the custom x axis
           self.ui(ui);
       });

        // needed in order to request repaint
        // ctx.request_repaint();

        ctx.request_repaint_after(std::time::Duration::from_secs_f32(
            1.0,
        ));
    }

    fn name(&self) -> &str {
        "Trader Balance"
    }
}

