use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::num::ParseFloatError;

const MARGIN: f64 = 0.0;

#[derive(Debug)]
pub enum CanvasError {
    /// try to write out of range
    OutOfRange(String),
    /// No data to plot
    NoData,
}

impl Display for CanvasError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::OutOfRange(s) => write!(f, "Canvas Error: Out of range {}", s),
            Self::NoData => write!(f, "Canvas Error: No Data"),
        }
    }
}

impl Error for CanvasError {}

#[derive(Debug)]
pub enum DatasetError {
    /// NoData
    NoData,
    /// Invalid data
    InvalidData(String),
}

impl Display for DatasetError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoData => write!(f, "Data Error: No data"),
            Self::InvalidData(s) => write!(f, "Data Error: Invalid Data: {}", s),
        }
    }
}

impl Error for DatasetError {}

impl From<ParseFloatError> for DatasetError {
    fn from(err: ParseFloatError) -> DatasetError {
        Self::InvalidData(err.to_string())
    }
}

/// Where to plot
#[derive(Default, Debug)]
pub struct Canvas {
    /// Vec<line: Vec<u8>>
    cells: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    x_range: (f64, f64),
    y_range: (f64, f64),
}

impl Canvas {
    pub fn new() -> Self {
        Self::from_size(25, 80)
    }

    fn from_size(height: usize, width: usize) -> Self {
        Self {
            cells: (0..height).map(|_| vec![b' '; width]).collect(),
            width,
            height,
            x_range: (0.0, 0.0),
            y_range: (0.0, 0.0),
        }
    }

    /// Remove drawing
    pub fn clear(&mut self) {
        self.cells = (0..self.height).map(|_| vec![b' '; self.width]).collect()
    }

    fn set_x_range(&mut self, x_min: f64, x_max: f64) {
        assert!(x_min < x_max);
        let delta = x_max - x_min;
        let x_range = (x_min - MARGIN * delta, x_max + MARGIN * delta);
        let cell_width = (x_range.1 - x_range.0) / self.width as f64;
        self.x_range = (
            x_min - (MARGIN * delta) - cell_width,
            x_max + (MARGIN * delta) + cell_width,
        );
    }

    fn set_y_range(&mut self, y_min: f64, y_max: f64) {
        assert!(y_min < y_max);
        let delta = y_max - y_min;
        let y_range = (y_min - MARGIN * delta, y_max + MARGIN * delta);
        let cell_width = (y_range.1 - y_range.0) / self.height as f64;
        self.y_range = (
            y_min - MARGIN * delta - 2.0 * cell_width,
            y_max + MARGIN * delta,
        );
    }

    /// Put a specific value in a specific cell
    fn set_cell(&mut self, line: usize, column: usize, value: u8) -> Result<(), CanvasError> {
        if let Some(cell) = self
            .cells
            .get_mut(line)
            .unwrap_or(&mut Vec::new())
            .get_mut(column)
        {
            *cell = value;
            Ok(())
        } else {
            Err(CanvasError::OutOfRange(format!(
                "try to write in ({}, {}) (Canvas size: ({}, {}))",
                line, column, &self.height, &self.width
            )))
        }
    }

    fn get_mut_cell(&mut self, line: usize, column: usize) -> Option<&mut u8> {
        todo!()
    }

    /// Put a specific value with specific coordinates in the canvas
    fn draw_value(&mut self, x: f64, y: f64, value: u8) -> Result<(), CanvasError> {
        self.set_cell(
            self.height - get_cell(y, self.y_range.0, self.y_range.1, self.height)?,
            get_cell(x, self.x_range.0, self.x_range.1, self.width)?,
            value,
        )
    }

    fn get_mut_value(&mut self, x: f64, y: f64) -> Option<&mut u8> {
        let offset = get_cell(y, self.y_range.0, self.y_range.1, self.height).ok()?;
        let line = self.height - offset;
        let column = get_cell(x, self.x_range.0, self.x_range.1, self.width).ok()?;
        self.get_mut_cell(line, column)
    }

    /// Draw axes
    fn draw_axes(&mut self) -> Result<(), CanvasError> {
        let y_axis_location = match get_cell(0.0, self.x_range.0, self.x_range.1, self.width) {
            Ok(u) => u,
            _ => {
                if self.x_range.1 < 0.0 {
                    self.width - 1
                } else {
                    0
                }
            }
        };
        let x_axis_location = match get_cell(0.0, self.y_range.0, self.y_range.1, self.height) {
            Ok(u) => u,
            _ => {
                if self.y_range.1 < 0.0 {
                    0
                } else {
                    self.height - 1
                }
            }
        };
        for cell in 0..self.width {
            let c = match (cell as i32 - y_axis_location as i32) % 5 {
                0 => b'+',
                _ => b'-',
            };
            self.set_cell(x_axis_location, cell, c)?;
        }
        for cell in 0..self.height {
            let c = match (cell as i32 - x_axis_location as i32) % 5 {
                0 => b'+',
                _ => b'|',
            };
            self.set_cell(cell, y_axis_location, c)?;
        }
        self.set_cell(x_axis_location, y_axis_location, b'+')?;
        Ok(())
    }
}

/// Get cell coordinate to write to
fn get_cell(x: f64, x_min: f64, x_max: f64, width: usize) -> Result<usize, CanvasError> {
    assert!(x_max > x_min);
    if x < x_min || x > x_max {
        Err(CanvasError::OutOfRange(format!("{x_min} < {x} < {x_max}")))
    } else {
        Ok(((width - 1) as f64 / (x_max - x_min) * (x - x_min)).round() as usize)
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for line in &self.cells {
            writeln!(
                f,
                "{}",
                line.iter().map(|&c| { c as char }).collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum PlotKind {
    #[default]
    Point,
    Boxplot,
    CDF,
    Histogram,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Quantiles {
    min: f64,
    q1: f64,
    q2: f64,
    q3: f64,
    max: f64,
    outliers: Vec<f64>,
}

impl Quantiles {
    pub fn from_slice(input: &[f64]) -> Self {
        let inter_quartiles = 1.5;
        let mut x: Vec<f64> = input.iter().filter(|a| !a.is_nan()).copied().collect();
        assert!(
            !x.is_empty(),
            "not enough valid values in input ({input:?})"
        );
        x.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let [q1, q2, q3] =
            [0.25, 0.5, 0.75].map(|q| get_value(x.as_slice(), get_index(q, x.len())).unwrap());
        let lower = q2 - inter_quartiles * (q3 - q1);
        let upper = q2 + inter_quartiles * (q3 - q1);
        Self {
            min: x
                .iter()
                .filter(|&a| *a > lower)
                .copied()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            q1,
            q2,
            q3,
            max: x
                .iter()
                .filter(|&a| *a < upper)
                .copied()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            outliers: x
                .iter()
                .filter(|&a| *a < lower || *a > upper)
                .copied()
                .collect(),
        }
    }

    /// Draw a horizontal boxplot on the canvas from lines height to height+3
    pub fn draw_into(&self, canvas: &mut Canvas, height: usize) -> Result<(), CanvasError> {
        assert!(canvas.height >= height + 3);
        let [min, q1, q2, q3, max] = [self.min, self.q1, self.q2, self.q3, self.max]
            .map(|x| get_cell(x, canvas.x_range.0, canvas.x_range.1, canvas.width));
        let outliers = self
            .outliers
            .iter()
            .map(|&x| get_cell(x, canvas.x_range.0, canvas.x_range.1, canvas.width))
            .collect::<Vec<_>>();

        let (q1,q2,  q3) = (q1?,q2?, q3?);
        let (min, max) = (min?, max?);
        for x in (min+1)..q1 {
            canvas.set_cell(height + 1 , x, b'-')?;
        }
        for x in (q3+1)..max {
            canvas.set_cell(height + 1 , x, b'-')?;
        }
        for x in outliers {
            canvas.set_cell(height + 1, x?, b'+')?;
        }
        for x in q1..q3 {
            canvas.set_cell(height, x, b'-')?;
            canvas.set_cell(height + 2, x, b'-')?;
        }
        for x in [min, q1, q2, q3, max] {
            canvas.set_cell(height + 1, x, b'|')?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct CDF {
    steps: Vec<(f64, f64)>,
}

impl CDF {
    pub fn from_vec(&mut self, input: Vec<f64>) -> Self {
        let step = 1.0 / (input.len() as f64);
        let mut steps: Vec<(f64, f64)> = Vec::new();
        let mut input: Vec<f64> = input
            .iter()
            .filter_map(|y| if y.is_nan() { None } else { Some(y) })
            .copied()
            .collect();
        input.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for y in input {
            if let Some(point) = steps.iter_mut().find(|elt| elt.0 == y) {
                point.1 += step
            } else {
                steps.push((y, step));
            }
        }
        Self { steps }
    }

    pub fn draw_into(&self, canvas: &mut Canvas) -> Result<(), CanvasError> {
        // XXX
        todo!()
    }

    /// Get the value of the CDF evaluted on x
    fn get_value(&self, x: f64) -> f64 {
        let mut y = 0.0;
        for p in &self.steps {
            if p.0 < x {
                y = p.1;
            }
        }
        y
    }
}

#[derive(Debug, Default)]
pub struct Histogram {
    /// Bins boundaries
    bins: Vec<f64>,
    /// number of sample per bins
    values: Vec<usize>,
}

impl Histogram {
    pub fn from_vec(&mut self, input: Vec<f64>) -> Self {
        if input.is_empty() {
            return Self::default();
        }
        let first = input[0];
        let (x_min, x_max) = input
            .iter()
            .copied()
            .fold((first, first), |(mi, ma), x| (x.min(mi), x.max(ma)));
        let x_max = x_max + 0.001 * (x_max - x_min);
        let mut hist = Histogram::default();
        hist.adjust_bins(x_min, x_max, 10);
        for x in input {
            todo!();
        }
        todo!()
    }

    pub fn draw_into(&self, canvas: &mut Canvas) -> Result<(), CanvasError> {
        todo!()
    }

    /// Get the value of the histogram at specific value
    fn get_value(&self, x: f64) -> f64 {
        todo!()
    }

    /// Get the normalized value of the histogram at specific value
    fn get_frequency(&self, x: f64) -> f64 {
        todo!()
    }

    /// Compute bins boundaries.
    fn adjust_bins(&mut self, x_min: f64, x_max: f64, bin_nb: usize) {
        let bin_size = x_max - x_min;
        self.bins = (0..=bin_nb).map(|x| x_min + x as f64 * bin_size).collect();
    }
}

#[derive(Debug, Default)]
pub struct DataSet {
    /// label: list of points
    dataset: HashMap<String, Vec<(f64, f64)>>,
}

impl DataSet {
    /// Build the dataset from the content of a csv file
    ///
    /// the content looks like
    /// ```plaintext
    ///      , A , B , "C"
    ///  -1  , 0 , 1 , 3
    ///  -5  , 1 , -2, 4
    /// ```
    pub fn from_csv(content: &str) -> Result<Self, DatasetError> {
        let sep = ',';
        let mut dataset = Self::default();
        let mut lines = content.lines();
        let headers: Vec<_> = lines
            .next()
            .ok_or(DatasetError::NoData)?
            .split(sep)
            .map(|l| String::from(l.replace('"', "").trim()))
            .skip(1)
            .collect();
        while let Some(line) = lines.next() {
            let mut values = line
                .split(sep)
                .map(|l| String::from(l.replace('"', "").trim()));
            let x = values
                .next()
                .expect("first column (indexes) should exist")
                .parse()?;
            for (label, y) in zip(headers.clone(), values) {
                dataset
                    .dataset
                    .entry(label.into())
                    .or_default()
                    .push((x, y.parse()?));
            }
        }
        Ok(dataset)
    }

    pub fn add_points(&mut self, dataset: String, points: Vec<(f64, f64)>) {
        self.dataset
            .entry(dataset)
            .or_default()
            .extend(points.iter())
    }

    pub fn draw_into(&self, canvas: &mut Canvas, kind: PlotKind) -> Result<(), CanvasError> {
        match kind {
            PlotKind::Point => self.draw_point(canvas),
            PlotKind::Boxplot => self.draw_boxplot(canvas),
            PlotKind::CDF => todo!(),
            PlotKind::Histogram => todo!(),
        }
    }

    fn draw_point(&self, canvas: &mut Canvas) -> Result<(), CanvasError> {
        // TODO check if range already set
        self.reset_canvas_range(canvas)?;
        canvas.draw_axes()?;

        // TODO add labels
        for (label, points) in self.dataset.iter() {
            // TODO: use correct labels
            let l = label.bytes().next().unwrap();
            for point in points {
                if point.0.is_nan() || point.1.is_nan() {
                    continue;
                }
                canvas.draw_value(point.0, point.1, l)?;
            }
        }
        Ok(())
    }

    fn draw_boxplot(&self, canvas: &mut Canvas) -> Result<(), CanvasError> {
        // TODO set canvas size
        let mut height = 0;
        for dataset in self.dataset.values() {
            let q = Quantiles::from_slice(&dataset.iter().map(|x| x.1).collect::<Vec<_>>());
            q.draw_into(canvas, height)?;
            height += 4
        }
        Ok(())
    }

    fn reset_canvas_range(&self, canvas: &mut Canvas) -> Result<(), CanvasError> {
        let mut points = self.dataset.values().flatten();
        let first = points.next().ok_or(CanvasError::NoData)?;
        let (x_min, x_max, y_min, y_max) = points.into_iter().fold(
            (first.0, first.0, first.1, first.1),
            |(x0, x1, y0, y1), p| (x0.min(p.0), x1.max(p.0), y0.min(p.1), y1.max(p.1)),
        );
        canvas.set_x_range(x_min, x_max);
        canvas.set_y_range(y_min, y_max);
        Ok(())
    }

    /// Get quantiles for each dataset
    fn get_quantiles(&self) -> HashMap<String, Option<Quantiles>> {
        todo!()
    }

    /// Get cumulative distribution for each dataset
    /// Return points where the distribution changes
    fn get_cumulatives(&self) -> HashMap<String, Option<Vec<(f64, f64)>>> {
        todo!()
    }
}

fn get_index(quantile: f64, length: usize) -> f64 {
    quantile * length as f64
}

/// Get value at specific non-integer index
///
/// Return a weighted sum of previous and next values
/// The nearest from an index, the most weight this index has
fn get_value(x: &[f64], idx: f64) -> Option<f64> {
    if idx + 1.0 > x.len() as f64 {
        return None;
    }
    assert!(x.len() as f64 >= idx);
    if idx == x.len() as f64 - 1.0 {
        return Some(*x.last().unwrap());
    }
    let f = idx.fract();
    let i = idx.floor() as usize;
    return Some((1.0 - f) * x[i] + f *(x[i + 1]));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_getter() {
        let v = [-1.0, 1.0];
        assert_eq!(get_value(&v, 0.0).unwrap(), -1.0);
        assert_eq!(get_value(&v, 1.0).unwrap(), 1.0);
        assert_eq!(get_value(&v, 0.5).unwrap(), 0.0);
        assert_eq!(get_value(&v, 0.25).unwrap(), -0.5);

        let v = [-1.0, 1.0, 2.0];
        assert!(get_value(&v, 2.1).is_none());
        assert_eq!(get_value(&v, 0.0).unwrap(), -1.0);
        assert_eq!(get_value(&v, 1.0).unwrap(), 1.0);
        assert_eq!(get_value(&v, 0.25).unwrap(), -0.5);
        assert_eq!(get_value(&v, 0.5).unwrap(), 0.0);

        let v = [];
        assert!(get_value(&v, 0.25).is_none());
    }

    #[test]
    fn quantiles() {
        let v = [1.0, 3.0, 4.0, 0.0, 2.0];
        let q = Quantiles::from_slice(&v);
        assert_eq!(
            q,
            Quantiles {
                min: 0.0,
                q1: 1.25,
                q2: 2.5,
                q3: 3.75,
                max: 4.0,
                outliers: Vec::new(),
            }
        );
    }

    #[test]
    fn dataset_csv() {
        let text = r#"
         , A , B , "C"
        -1  , 0 , 1 , 3
        -5  , 1 , -2, 4
    "#
        .trim();
        let dataset = DataSet::from_csv(text).unwrap();
        dbg!(dataset);
    }
}
