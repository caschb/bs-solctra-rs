use clap::{Parser, error::Result};
use constants::{MAJOR_RADIUS, MINOR_RADIUS};
use log::{debug, info, trace};
use point::{Point, Points};
use std::{
    fs::{self, DirBuilder, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

mod constants;
mod point;

fn read_from_file(path: &Path) -> Points {
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to resource folder
    #[arg(long)]
    resource_path: String,

    /// Particles file
    #[arg(long)]
    particles_file: String,

    /// Total simulation steps
    #[arg(long, default_value_t = 10000)]
    steps: u32,

    /// Size of time step
    #[arg(long, default_value_t = 0.001)]
    step_size: f64,

    /// Precision to use
    #[arg(long, default_value_t = 5)]
    precision: u8,

    /// Total particles to use
    #[arg(long, default_value_t = 1)]
    length: u32,

    /// Mode to run
    #[arg(long, default_value_t = 1)]
    mode: u8,

    /// Magnetic profile
    #[arg(long, default_value_t = 0)]
    magprof: u8,

    /// Total points
    #[arg(long, default_value_t = 10000)]
    num_points: u32,

    /// Phi angle
    #[arg(long, default_value_t = 0)]
    phi_angle: u32,

    /// Dimension
    #[arg(long, default_value_t = 1)]
    dimension: u8,

    /// Output directory
    #[arg(long)]
    output: String,
}

fn create_directory(path: &Path) {
    let dirbuilder = DirBuilder::new();
    info!("Creating path: {}", path.display());
    match dirbuilder.create(path) {
        Ok(_) => debug!("Succesfully created directory: {}", path.display()),
        Err(_) => todo!(),
    }
}

fn compute_magnetic_field(
    particle: &Point,
    coils: &Vec<Points>,
    displacements: &Vec<Points>,
    e_roof: &Vec<Points>,
) -> Point {
    let multiplier = (constants::MIU * constants::I) / (4.0 * constants::PI);
    let mut bx = 0.0;
    let mut by = 0.0;
    let mut bz = 0.0;

    for coil_idx in 0..coils.len() {
        for point_idx in 0..coils[coil_idx].len() - 1 {
            let rmi_a = particle.get_displacement(&coils[coil_idx][point_idx]);
            let rmf_a = particle.get_displacement(&coils[coil_idx][point_idx + 1]);
            let u = Point {
                x: multiplier * e_roof[coil_idx][point_idx].x,
                y: multiplier * e_roof[coil_idx][point_idx].y,
                z: multiplier * e_roof[coil_idx][point_idx].z,
            };
            let displacement_norm = displacements[coil_idx][point_idx].get_norm();
            let rmi_a_norm = rmi_a.get_norm();
            let rmf_a_norm = rmf_a.get_norm();
            let c = ((2.0 * displacement_norm * (rmi_a_norm + rmf_a_norm))
                / (rmi_a_norm * rmf_a_norm))
                * ((1.0)
                    / ((rmi_a_norm + rmf_a_norm) * (rmi_a_norm + rmf_a_norm)
                        - displacement_norm * displacement_norm));
            let v = Point {
                x: rmi_a.x * c,
                y: rmi_a.y * c,
                z: rmi_a.z * c,
            };
            bx = bx + (u.y * v.z) - (u.z * v.y);
            by = by + (u.z * v.x) + (u.x * v.z);
            bz = bz + (u.x * v.y) - (u.y * v.x);
        }
    }
    let b = Point {
        x: bx,
        y: by,
        z: bz,
    };
    b
}

fn simulate_step(
    particle: &Point,
    coils: &Vec<Points>,
    displacements: &Vec<Points>,
    e_roof: &Vec<Points>,
    step_size: f64,
) -> Point {
    let mut k1 = compute_magnetic_field(particle, coils, displacements, e_roof);
    let k1norm = k1.get_norm();
    k1.x = (k1.x / k1norm) * step_size;
    k1.y = (k1.y / k1norm) * step_size;
    k1.z = (k1.z / k1norm) * step_size;
    let p1 = Point {
        x: k1.x / 2.0 + particle.x,
        y: k1.y / 2.0 + particle.y,
        z: k1.z / 2.0 + particle.z,
    };

    let mut k2 = compute_magnetic_field(&p1, coils, displacements, e_roof);
    let k2norm = k2.get_norm();
    k2.x = (k2.x / k2norm) * step_size;
    k2.y = (k2.y / k2norm) * step_size;
    k2.z = (k2.z / k2norm) * step_size;
    let p2 = Point {
        x: k2.x / 2.0 + particle.x,
        y: k2.y / 2.0 + particle.y,
        z: k2.z / 2.0 + particle.z,
    };

    let mut k3 = compute_magnetic_field(&p2, coils, displacements, e_roof);
    let k3norm = k3.get_norm();
    k3.x = (k3.x / k3norm) * step_size;
    k3.y = (k3.y / k3norm) * step_size;
    k3.z = (k3.z / k3norm) * step_size;
    let p3 = Point {
        x: k3.x + particle.x,
        y: k3.y + particle.y,
        z: k3.z + particle.z,
    };
    let mut k4 = compute_magnetic_field(&p3, coils, displacements, e_roof);
    let k4norm = k4.get_norm();
    k4.x = (k4.x / k4norm) * step_size;
    k4.y = (k4.y / k4norm) * step_size;
    k4.z = (k4.z / k4norm) * step_size;
    let mut result = Point {
        x: particle.x + (k1.x + 2.0 * k2.x + 2.0 * k3.x + k4.x) / 6.0,
        y: particle.y + (k1.y + 2.0 * k2.y + 2.0 * k3.y + k4.y) / 6.0,
        z: particle.z + (k1.z + 2.0 * k2.z + 2.0 * k3.z + k4.z) / 6.0,
    };

    let origin = Point {
        x: MAJOR_RADIUS * result.x / result.get_norm(),
        y: MAJOR_RADIUS * result.y / result.get_norm(),
        z: 0.0,
    };

    let distance = result.get_distance(&origin);
    if distance > MINOR_RADIUS {
        result.x = MINOR_RADIUS;
        result.y = MINOR_RADIUS;
        result.z = MINOR_RADIUS;
    }

    result
}

fn simulate_particles(
    particles: &mut [Point],
    total_steps: u32,
    step_size: f64,
    coils: &Vec<Points>,
    displacements: &Vec<Points>,
    e_roof: &Vec<Points>,
    output_dir: &Path,
) {
    let length = particles.len();
    let divergent_particle = Point {
        x: MINOR_RADIUS,
        y: MINOR_RADIUS,
        z: MINOR_RADIUS,
    };

    debug!("Total particles: {}", length);

    write_points_to_file(&particles, output_dir, 0);
    for step in 1..total_steps + 1 {
        for particle in &mut *particles {
            if *particle == divergent_particle {
                continue;
            } else {
                *particle = simulate_step(particle, coils, displacements, e_roof, step_size);
            }
        }
        if step % 10 == 0 {
            write_points_to_file(&particles, output_dir, step);
        }
    }
}

fn read_coil_data_directory(path: &Path) -> Vec<Points> {
    let mut coil_files = fs::read_dir(path)
        .expect("Error reading file")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    coil_files.sort();

    let mut coils = Vec::<Points>::new();

    for coil_file in coil_files {
        let data = read_from_file(&coil_file.as_path());
        coils.push(data);
    }
    debug!("Read {} coils", coils.len());
    return coils;
}

fn compute_displacements(coil: &[Point]) -> Points {
    coil.windows(2)
        .map(|w| w[1].get_displacement(&w[0]))
        .collect()
}

fn compute_all_displacements(coils: &Vec<Points>) -> Vec<Points> {
    coils.iter().map(|c| compute_displacements(c)).collect()
}

fn compute_e_roof(coil_displacements: &[Point]) -> Points {
    coil_displacements
        .iter()
        .map(|disp| disp.get_unit_vector())
        .collect()
}

fn compute_all_e_roof(all_displacements: &Vec<Points>) -> Vec<Points> {
    all_displacements
        .iter()
        .map(|disps| compute_e_roof(disps))
        .collect()
}

fn write_points_to_file(particles: &[Point], output_dir: &Path, step: u32) {
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

fn main() {
    let args = Args::parse();
    env_logger::init();
    info!("Starting BS-Solctra");
    debug!("{:?}", args);
    let output_dir = Path::new(&args.output);

    match fs::exists(output_dir) {
        Ok(true) => info!("Output path: {} already exists", args.output),
        Ok(false) => create_directory(output_dir),
        Err(_) => panic!("Error querying path"),
    }
    info!("Reading particles from file {}", args.particles_file);
    let mut particles = read_from_file(Path::new(&args.particles_file));

    info!("Reading coil data from directory: {}", &args.resource_path);
    let coils = read_coil_data_directory(Path::new(&args.resource_path));
    let displacements = compute_all_displacements(&coils);
    let e_roof = compute_all_e_roof(&displacements);

    simulate_particles(
        particles.as_mut_slice(),
        args.steps,
        args.step_size,
        &coils,
        &displacements,
        &e_roof,
        output_dir,
    );
}
