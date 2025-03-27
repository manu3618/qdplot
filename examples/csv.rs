use qdplot::{Canvas, DataSet, PlotKind};
use std::fs;

fn main() {
    let dataset =
        DataSet::from_csv(fs::read_to_string("examples/sample.csv").unwrap().as_str()).unwrap();
    let mut canvas = Canvas::new();
    let _ = dataset.draw_into(&mut canvas, PlotKind::default());
    println!("{canvas}");
}
