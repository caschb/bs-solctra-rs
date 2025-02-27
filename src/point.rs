use core::fmt;

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
