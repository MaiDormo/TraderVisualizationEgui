mod app;
mod balance;
mod panel;

use crate::app::App;
use crate::balance::Currency;
use eframe::egui::plot::PlotPoint;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};
use tracing::{error, info, warn};

fn main() {
    // Force winit to use X11 on Unix before winit/eframe/winit initialize.
    // This can avoid freetype/crossfont panics on some Wayland setups.
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    let app = App::new();
    let options = eframe::NativeOptions::from(app.options.clone());

    let path = "./src/values.txt".to_string();
    let monitor_ref = app.balance_measurements.clone();
    let mut current_day = 0;

    thread::spawn(move || {
        //iterate strategy for etc...
        //inside il for si aggiunge values
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
                        .add(PlotPoint::new(current_day, usd), Currency::USD);
                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day, eur), Currency::EUR);
                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day, yen), Currency::YEN);
                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(PlotPoint::new(current_day, yuan), Currency::YUAN);

                    //update current day
                    current_day += 1;

                    //sleep the thread so it can happen in a more visual manner
                    thread::sleep(time::Duration::from_secs(1));
                }
                _ => {
                    error!("Failed to read the line");
                    break;
                }
            }
        }
    });

    info!("Main thread started");
    eframe::run_native(" Trader ", options, Box::new(|_cc| Box::new(app)))
}
