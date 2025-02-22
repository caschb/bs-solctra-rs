use clap::Parser;
use log::{debug, info};
use std::{
    fs::{self, DirBuilder, File},
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Point<N = f64>(N, N, N);

fn read_particles(path: &Path) -> Vec<Point> {
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
        let parsed_parts: Vec<f64> = parts.map(|x| x.trim().parse::<f64>().unwrap()).collect();
        let x = parsed_parts[0];
        let y = parsed_parts[1];
        let z = parsed_parts[2];
        let point = Point(x, y, z);
        points.push(point);
    }
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

fn simulate_particles(particles: Vec<Point>, total_steps: u32, step_size: f64) {
    let length = particles.len();

    debug!("Total particles: {}", length);

    for step in 0..total_steps {
        for particle in &particles {
            debug!("{} {:?}", step, particle)
        }
    }
}

fn main() {
    let args = Args::parse();
    env_logger::init();
    info!("Starting BS-Solctra");
    debug!("{:?}", args);
    let path = Path::new(&args.output);

    match fs::exists(path) {
        Ok(true) => info!("Output path: {} already exists", args.output),
        Ok(false) => create_directory(path),
        Err(_) => panic!("Error querying path"),
    }
    info!("Reading files from file {}", args.particles_file);
    let particles = read_particles(Path::new(&args.particles_file));
    debug!("Successfully read particles");

    simulate_particles(particles, args.steps, args.step_size);
}
