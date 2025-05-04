use clap::Parser;
use qdplot::{Canvas, DataSet, PlotKind};
use std::fs;
use std::path::PathBuf;

/// Tool to quickly plot dataset
///
/// Tool highly inspired from guff (https://github.com/silentbicycle/guff)
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// input CSV file
    input: PathBuf,

    /// Plotkind
    #[arg(short, long, default_value_t=Default::default())]
    kind: PlotKind,
}

fn main() {
    let args = Args::parse();
    let dataset = DataSet::from_csv(fs::read_to_string(args.input).unwrap().as_str()).unwrap();
    let mut canvas = Canvas::new();
    let _ = dataset.draw_into(&mut canvas, args.kind);
    println!("{canvas}");
}
