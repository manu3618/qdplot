use qdplot::{Canvas,DataSet};

fn main() {
    let mut dataset = DataSet::default();
    dataset.add_points("1".into(), vec![(0.0, 0.0), (0.0, 1.0), (-4.0, 4.0), (-3.3, -2.5)]);
    dataset.add_points("2".into(), vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]);
    dataset.add_points("3".into(), vec![(0.0, 0.0), (-1.0, -1.0), (-2.0, -2.0), (-3.0, -3.0)]);
    dataset.add_points("4".into(), vec![(0.0, 0.0)]);
    let mut canvas = Canvas::new();
    let _  = dataset.draw_into(&mut canvas);
    println!("{canvas}");
}
