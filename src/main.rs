mod balance;
use std::ops::RangeInclusive;
use std::{thread, time};
use std::default::Default;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use eframe::{egui, HardwareAcceleration, Theme};
use eframe::egui::plot::{CoordinatesFormatter, Corner, Legend, PlotPoint};
use eframe::egui::{Align, Color32, Response, Ui, Vec2};
use egui::plot::{Line, Plot};
use tracing::{error, info, warn};


use crate::balance::{BalanceMeasurements, Currency};


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
        resizable: true,
        transparent: true,
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
        "Trader ",
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

#[derive(Default)]
struct App {
    balance_measurements: Arc<Mutex<BalanceMeasurements>>,
    open_panel: Panel
}

impl App {
    fn new() -> Self {
        Self {
            balance_measurements: Arc::new(Mutex::new(BalanceMeasurements::new())),
            open_panel: Default::default()
        }
    }

    fn ui(&mut self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
                ui.label("Merge: Gives the options to see the balance of all the current currencies");
                ui.label("Divide: Gives the options to see the individual balance for all currencies");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.open_panel, Panel::Merged, "Merged");
                ui.selectable_value(&mut self.open_panel, Panel::Divided, "Divided");
            });
            ui.separator();


            //fixed color for all currency
            let usd_color = Color32::from_rgb(51,255,51);
            let eur_color = Color32::from_rgb(51,51,255);
            let yen_color = Color32::from_rgb(255,51,51);
            let yuan_color = Color32::from_rgb(255,255,51);

            //format the x Axis
            let x_fmt = |x:f64, _range: &RangeInclusive<f64>| {
                // Days
                format!("Day {}", x)
            };

            match self.open_panel {
                Panel::Merged => {
                    let plot = Plot::new("Merged Graph");
                    //code required to plot
                    plot
                        //to add a Legend
                        .legend(Legend::default())

                        //to custom format the axis
                        .x_axis_formatter(x_fmt)

                        //to add a coordinate show box in the right bottom of the screen
                        .coordinates_formatter(Corner::RightBottom, CoordinatesFormatter::default())

                        //to show the UI
                        .show(ui,|plot_ui| {
                            let usd = Line::new(self.balance_measurements.lock().unwrap().plot_values_usd());
                            let eur = Line::new(self.balance_measurements.lock().unwrap().plot_values_eur());
                            let yen = Line::new(self.balance_measurements.lock().unwrap().plot_values_yen());
                            let yuan = Line::new(self.balance_measurements.lock().unwrap().plot_values_yuan());
                            plot_ui.line(usd.width(3.0).color(Color32::from(usd_color)).name("USD"));
                            plot_ui.line(eur.width(3.0).color(Color32::from(eur_color)).name("EUR"));
                            plot_ui.line(yen.width(3.0).color(Color32::from(yen_color)).name("YEN"));
                            plot_ui.line(yuan.width(3.0).color(Color32::from(yuan_color)).name("YUAN"));
                        })
                }
                Panel::Divided => {
                    ui.vertical_centered(|ui| {
                        //instantiating all the plotting
                        let plot_usd = Plot::new("USD");
                        let plot_eur = Plot::new("EUR");
                        let plot_yen = Plot::new("YEN");
                        let plot_yuan = Plot::new("YUAN");

                        //modding plotting for the USD
                        plot_usd
                            .height(150.0)
                            .legend(Legend::default())
                            .x_axis_formatter(x_fmt)
                            .coordinates_formatter(Corner::RightBottom, CoordinatesFormatter::default())
                            .show(ui, |plot_ui| {
                                let usd = Line::new(self.balance_measurements.lock().unwrap().plot_values_usd());
                                plot_ui.line(
                                    usd.width(3.0).color(Color32::from(usd_color)).name("USD"));
                            });

                        ui.separator();

                        //modding plotting for the EUR
                        plot_eur
                            .height(150.0)
                            .legend(Legend::default())
                            .x_axis_formatter(x_fmt)
                            .coordinates_formatter(Corner::RightBottom, CoordinatesFormatter::default())
                            .show(ui, |plot_ui| {
                                let usd = Line::new(self.balance_measurements.lock().unwrap().plot_values_eur());
                                plot_ui.line(usd.width(3.0).color(Color32::from(eur_color)).name("EUR"));
                            });

                        ui.separator();

                        //modding plotting for the YEN
                        plot_yen
                            .height(150.0)
                            .legend(Legend::default())
                            .x_axis_formatter(x_fmt)
                            .coordinates_formatter(Corner::RightBottom, CoordinatesFormatter::default())
                            .show(ui, |plot_ui| {
                                let usd = Line::new(self.balance_measurements.lock().unwrap().plot_values_yen());
                                plot_ui.line(usd.width(3.0).color(Color32::from(yen_color)).name("YEN"));
                            });

                        ui.separator();

                        //modding plotting for the YUAN
                        plot_yuan
                            .height(150.0)
                            .legend(Legend::default())
                            .x_axis_formatter(x_fmt)
                            .coordinates_formatter(Corner::RightBottom, CoordinatesFormatter::default())
                            .show(ui, |plot_ui| {
                                let usd = Line::new(self.balance_measurements.lock().unwrap().plot_values_yuan());
                                plot_ui.line(usd.width(3.0).color(Color32::from(yuan_color)).name("YUAN"));
                            });
                    })

                }
            }
    })
            .response
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           //code for the custom x axis
           self.ui(ui);
       });

        //needed in order to request repaint
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Trader Balance"
    }
}

