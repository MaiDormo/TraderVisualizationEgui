use std::collections::VecDeque;
use eframe::egui::plot::{PlotPoint, PlotPoints};
use std::thread;


//try
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;


pub enum Currency {
    USD,
    EUR,
    YEN,
    YUAN
}

#[derive(Default)]
pub struct BalanceMeasurements {
    pub usd: VecDeque<PlotPoint>,
    pub eur: VecDeque<PlotPoint>,
    pub yen: VecDeque<PlotPoint>,
    pub yuan: VecDeque<PlotPoint>
}
impl BalanceMeasurements {
    pub fn new() -> Self {
        Self {
            usd: VecDeque::new(),
            eur: VecDeque::new(),
            yen: VecDeque::new(),
            yuan: VecDeque::new()
        }
    }

    pub fn append_values_usd(&mut self, v: PlotPoint) {
        self.usd.push_back(v);
    }
    pub fn append_values_eur(&mut self, v: PlotPoint) {
        self.eur.push_back(v);
    }
    pub fn append_values_yen(&mut self, v: PlotPoint) {
        self.yen.push_back(v);
    }
    pub fn append_values_yuan(&mut self, v: PlotPoint) {
        self.yuan.push_back(v);
    }


    pub fn plot_values_usd(&self) -> PlotPoints {
        PlotPoints::Owned(Vec::from_iter(self.usd.iter().copied()))
    }
    pub fn plot_values_eur(&self) -> PlotPoints {
        PlotPoints::Owned(Vec::from_iter(self.eur.iter().copied()))
    }
    pub fn plot_values_yen(&self) -> PlotPoints {
        PlotPoints::Owned(Vec::from_iter(self.yen.iter().copied()))
    }
    pub fn plot_values_yuan(&self) -> PlotPoints {
        PlotPoints::Owned(Vec::from_iter(self.yuan.iter().copied()))
    }

    pub fn add(&mut self, point:PlotPoint, val:Currency){
        match val {
            Currency::USD => {
                self.usd.push_back(point)
            }
            Currency::EUR => {
                self.eur.push_back(point)
            }
            Currency::YEN => {
                self.yen.push_back(point)
            }
            Currency::YUAN => {
                self.yuan.push_back(point)
            }
        };
    }

}