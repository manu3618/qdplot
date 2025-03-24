use qdplot::{Canvas, DataSet};

fn main() {
    let mut dataset = DataSet::default();
    dataset.add_points("1".into(), vec![(1.0, 1.0), (4.0, 4.0), (3.3, 2.5)]);
    let mut canvas = Canvas::new();
    let _ = dataset.draw_into(&mut canvas);
    println!("{canvas}");
}
