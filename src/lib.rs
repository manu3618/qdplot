use std::collections::HashMap;
use std::fmt::{Display, Formatter};

const MARGIN: f64 = 0.0;

#[derive(Debug)]
pub enum CanvasError {
    /// try to write out of range
    OutOfRange(String),
    /// No data to plot
    NoData,
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

#[derive(Debug, Default)]
pub struct DataSet {
    /// label: list of points
    dataset: HashMap<String, Vec<(f64, f64)>>,
}

impl DataSet {
    pub fn add_points(&mut self, dataset: String, points: Vec<(f64, f64)>) {
        self.dataset
            .entry(dataset)
            .or_default()
            .extend(points.iter())
    }

    pub fn draw_into(&self, canvas: &mut Canvas) -> Result<(), CanvasError> {
        // TODO check if range already set
        self.reset_canvas_range(canvas)?;
        canvas.draw_axes()?;

        // TODO add labels
        for (label, points) in self.dataset.iter() {
            // TODO: use correct labels
            let l = label.bytes().next().unwrap();
            for point in points {
                canvas.draw_value(point.0, point.1, l)?;
            }
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
}
