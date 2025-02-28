use clap::Parser;
use log::{debug, info, trace};
use std::{
    fs::{self, DirBuilder},
    path::Path,
};

use bs_solctra_rs::{args, point, simulation};

fn create_directory(path: &Path) {
    let dirbuilder = DirBuilder::new();
    info!("Creating path: {}", path.display());
    match dirbuilder.create(path) {
        Ok(_) => debug!("Succesfully created directory: {}", path.display()),
        Err(_) => todo!(),
    }
}

fn main() {
    env_logger::init();
    info!("Starting BS-Solctra");
    let args = args::Args::parse();
    debug!("{:?}", args);
    let output_dir = Path::new(&args.output);

    match fs::exists(output_dir) {
        Ok(true) => info!("Output path: {} already exists", args.output),
        Ok(false) => create_directory(output_dir),
        Err(err) => panic!("Error querying path: {}", err),
    }
    info!("Reading particles from file {}", args.particles_file);
    let max_particles = args.num_particles;
    let mut particles = match point::read_from_file(Path::new(&args.particles_file), max_particles)
    {
        Ok(particles) => particles,
        Err(err) => panic!("Error: {}", err),
    };

    info!("Reading coil data from directory: {}", &args.resource_path);
    let coils = match simulation::read_coil_data_directory(Path::new(&args.resource_path)) {
        Ok(coils) => coils,
        Err(err) => panic!("Error: {}", err),
    };
    info!("Computing displacements");
    let displacements = simulation::compute_all_displacements(&coils);
    info!("Computing e_roof");
    let e_roof = simulation::compute_all_e_roof(&displacements);
    debug!("Total e_roof: {}", e_roof.len());

    trace!("{:?}", e_roof);

    simulation::simulate_particles(
        particles.as_mut_slice(),
        args.steps,
        args.step_size,
        &coils,
        &displacements,
        &e_roof,
        output_dir,
    );
}
