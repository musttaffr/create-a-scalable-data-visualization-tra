// Import necessary libraries
extern crate chrono;
extern crate plotters;
extern crate rand;
use chrono::{Duration, Utc};
use plotters::prelude::*;
use rand::Rng;

// Define a struct to hold data points
struct DataPoint {
    timestamp: i64,
    value: f64,
}

// Define a struct to hold the tracker
struct ScalableTracker {
    data_points: Vec<DataPoint>,
    max_size: usize,
}

impl ScalableTracker {
    fn new(max_size: usize) -> ScalableTracker {
        ScalableTracker {
            data_points: Vec::new(),
            max_size,
        }
    }

    fn add_data_point(&mut self, timestamp: i64, value: f64) {
        self.data_points.push(DataPoint { timestamp, value });
        if self.data_points.len() > self.max_size {
            self.data_points.drain(..self.data_points.len() - self.max_size);
        }
    }

    fn visualize(&self) -> Vec<(i64, f64)> {
        self.data_points.clone().into_iter().collect()
    }
}

fn main() {
    // Create a new tracker with a maximum size of 100
    let mut tracker = ScalableTracker::new(100);

    // Generate random data points and add them to the tracker
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let timestamp = Utc::now().timestamp();
        let value = rng.gen_range(-100.0..100.0);
        tracker.add_data_point(timestamp, value);
    }

    // Visualize the data points
    let data_points = tracker.visualize();
    let root_area = BitMapBackend::new("scalable_tracker.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Scalable Tracker", ("sans-serif", 40).into_font())
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data_points.len() as u32, -100..100.0)
        .unwrap();
    chart
        .configure_mesh()
        .draw()
        .unwrap();
    chart.draw_series(LineSeries::new(
        data_points.into_iter().enumerate().map(|(i, (timestamp, value))| (i as u32, value)),
        &BLUE,
    ))
    .unwrap();
}