use core::fmt;
use csv;
use log::debug;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub(crate) struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Point {
    pub(crate) fn get_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub(crate) fn get_distance(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        (x * x + y * y + z * z).sqrt()
    }

    pub(crate) fn get_displacement(&self, other: &Point) -> Point {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Point { x, y, z }
    }

    pub(crate) fn get_unit_vector(&self) -> Point {
        let norm = self.get_norm();
        let x = self.x / norm;
        let y = self.y / norm;
        let z = self.z / norm;
        Point { x, y, z }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

pub(crate) fn read_from_file(path: &Path, max_items: usize) -> Result<Vec<Point>, Box<dyn Error>> {
    debug!("Reading data from file {:?}", path);
    let mut rdr = csv::Reader::from_path(path)?;
    let mut points = Vec::<Point>::new();
    for result in rdr.deserialize().take(max_items) {
        let point: Point = result?;
        points.push(point);
    }
    debug!("Read {} points from file {:?}", points.len(), path);
    return Ok(points);
}

pub(crate) fn write_points_to_file(
    points: &[Point],
    output_dir: &Path,
    step: u32,
) -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::new();
    path.push(output_dir);
    path.push(format!("out_{}.csv", step));
    let mut wtr = csv::Writer::from_path(path)?;
    // let mut wtr = csv::Writer::from_writer(vec![]);
    for point in points {
        wtr.serialize(point)?;
    }
    Ok(())
}
