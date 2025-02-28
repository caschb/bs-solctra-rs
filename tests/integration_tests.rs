use bs_solctra_rs::point::*;
use bs_solctra_rs::simulation::*;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;

#[test]
fn test_simulation() {
    let mut particle_vec = vec![Point {
        x: 0.14557183,
        y: 0.0,
        z: 0.0,
    }];

    let steps = 1u32;
    let step_size = 0.01;
    let output_path = Path::new("tests/test_output");
    match create_dir(output_path) {
        Ok(_) => println!("Sucessfully created directory: {:?}", output_path),
        Err(err) => panic!(
            "Error creating directory: {:?} due to error: {}",
            output_path, err
        ),
    };
    let coil_path = Path::new("./tests/test-resources/resources");
    let coils = match read_coil_data_directory(coil_path) {
        Ok(coils) => coils,
        Err(err) => panic!(
            "Error reading coils from directory {:?} due to error {}",
            coil_path, err
        ),
    };

    let displacements = compute_all_displacements(&coils);
    let e_roof = compute_all_e_roof(&displacements);
    let write_frequency = 1u32;

    simulate_particles(
        &mut particle_vec,
        steps,
        step_size,
        &coils,
        &displacements,
        &e_roof,
        output_path,
        write_frequency,
    );

    let output_particle = Point {
        x: 0.1455416056924451,
        y: 0.009491670745324678,
        z: 0.0031465260786825264,
    };

    let output_file = Path::new("tests/test_output/out_1.csv");

    let final_vector = match read_from_file(output_file, 1) {
        Ok(particles) => particles,
        Err(err) => panic!("Error: {}", err),
    };

    match remove_dir_all(output_path) {
        Ok(_) => println!("Successfully removed dir"),
        Err(err) => panic!("Error: {}", err),
    }

    let result = final_vector.iter().all(|v| *v == output_particle);
    assert!(result);
}
