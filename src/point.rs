use core::fmt;
use csv;
use log::debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::error::Error;

#[derive(Debug, Default, PartialEq, Clone, Copy, serde::Deserialize)]
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

pub(crate) fn read_from_file(path: &Path) -> Result<Vec<Point>, Box<dyn Error>> {
    debug!("Reading data from file {:?}", path);
    let mut rdr = csv::Reader::from_path(path)?;
    let mut points = Vec::<Point>::new();
    for result in rdr.deserialize() {
        let point: Point = result?;
        points.push(point);
    }
    debug!("Read {} points from file {:?}", points.len(), path);
    return Ok(points);
}

pub(crate) fn write_points_to_file(particles: &[Point], output_dir: &Path, step: u32) {
    let mut path = PathBuf::new();
    path.push(output_dir);
    path.push(format!("out_{}.csv", step));
    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(_) => panic!("Error creating file"),
    };
    f.write(b"x,y,z\n").unwrap();
    f.write(
        particles
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}
