use qdplot::{Canvas, DataSet, PlotKind};
use std::fs;

fn main() {
    let dataset =
        DataSet::from_csv(fs::read_to_string("examples/sample.csv").unwrap().as_str()).unwrap();
    let mut canvas = Canvas::new();
    let _ = dataset.draw_into(&mut canvas, PlotKind::Point);
    println!("{canvas}");
    canvas.clear();
    let _ = dataset.draw_into(&mut canvas, PlotKind::Boxplot);
    println!("{canvas}");
    canvas.clear();
    let _ = dataset.draw_into(&mut canvas, PlotKind::CDF);
    println!("{canvas}");
    canvas.clear();
    let _ = dataset.draw_into(&mut canvas, PlotKind::Histogram);
    println!("{canvas}");
}
