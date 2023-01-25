use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};
use eframe::egui::{Color32, Response, Ui};
use eframe::egui::plot::{CoordinatesFormatter, Corner, Legend, Line, Plot};
use crate::balance::BalanceMeasurements;
use crate::Panel;

#[derive(Default)]
pub struct App {
    pub balance_measurements: Arc<Mutex<BalanceMeasurements>>,
    open_panel: Panel
}

impl App {
    pub fn new() -> Self {
        Self {
            balance_measurements: Arc::new(Mutex::new(BalanceMeasurements::new())),
            open_panel: Default::default()
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Response {
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