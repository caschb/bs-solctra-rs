use core::fmt;
use log::{debug, trace};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
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

pub(crate) fn read_from_file(path: &Path) -> Vec<Point> {
    debug!("Reading data from file {:?}", path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => panic!("File {} not found!", path.display()),
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut points = Vec::<Point>::new();
    for line in lines {
        let vals = match line {
            Ok(values) => values,
            Err(_) => panic!("Error reading line from file {}", path.display()),
        };
        let parts = vals.split("\t");
        let parsed_parts: Vec<f64> = parts
            .map(|line| match line.trim().parse::<f64>() {
                Ok(value) => value,
                Err(_) => panic!("Error parsing string: {}", line),
            })
            .collect();
        let x = parsed_parts[0];
        let y = parsed_parts[1];
        let z = parsed_parts[2];
        let point = Point { x, y, z };
        trace!("{}: {:?}", path.display(), point);
        points.push(point);
    }
    debug!("Read {} points from {}", points.len(), path.display());
    return points;
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
