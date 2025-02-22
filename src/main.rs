use clap::{error::Result, Parser};
use log::{debug, info, trace};
use std::{
    fs::{self, DirBuilder, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Point
{
    x:f64, 
    y:f64, 
    z:f64
}

impl Point {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

fn read_from_file(path: &Path) -> Vec<Point> {
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
        let parsed_parts: Vec<f64> = parts.map(|line|{
            match line.trim().parse::<f64>() {
                Ok(value) => value,
                Err(_) => panic!("Error parsing string: {}", line),
            }
        }).collect();
        let x = parsed_parts[0];
        let y = parsed_parts[1];
        let z = parsed_parts[2];
        let point = Point{x, y, z};
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

fn simulate_particles(particles: Vec<Point>, total_steps: u32, step_size: f64) {
    let length = particles.len();

    debug!("Total particles: {}", length);

    for step in 0..total_steps {
        for particle in &particles {
            debug!("{} {:?}", step, particle)
        }
    }
}

fn read_coil_data_directory(path: &Path) -> Vec::<Vec::<Point>> {
   let mut coil_files = fs::read_dir(path).expect("Error reading file")
   .map(|res| res.map(|e| e.path()))
   .collect::<Result<Vec<_>, io::Error>>().unwrap();
   coil_files.sort();

   let mut coils = Vec::<Vec::<Point>>::new();

   for coil_file in coil_files {
        let data = read_from_file(&coil_file.as_path());
        coils.push(data);
   };
   debug!("Read {} coils", coils.len());
   return coils;
}

fn compute_e_roof(coils: &Vec::<Vec::<Point>>) -> Vec<Vec<Point>> {
    let mut e_roof = Vec::<Vec::<Point>>::new();
    let mut segment = Point { x: 0.0, y: 0.0, z: 0.0 };
    for (i, coil) in coils.iter().enumerate() {
        for degree in 0..coil.len()-1 {
            segment.x = coil[degree + 1].x - coil[degree].x;
            segment.y = coil[degree + 1].y - coil[degree].y;
            segment.z = coil[degree + 1].z - coil[degree].z;
            // TODO: collect length segments
            let length_segment = segment.norm();
            let x = segment.x / length_segment;
            let y = segment.y / length_segment;
            let z = segment.z / length_segment;
            // FIXME: I think vectors don't work like this
            e_roof[i].push(Point { x, y ,z });
        }
    }
    return e_roof;
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
    info!("Reading particles from file {}", args.particles_file);
    let particles = read_from_file(Path::new(&args.particles_file));

    info!("Reading coil data from directory: {}", &args.resource_path);
    let coils = read_coil_data_directory(Path::new(&args.resource_path));
    let e_roof = compute_e_roof(&coils);

    // simulate_particles(particles, args.steps, args.step_size);
}
