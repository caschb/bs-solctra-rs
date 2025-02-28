use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to resource folder
    #[arg(short, long)]
    pub resource_path: String,

    /// Particles file
    #[arg(short, long)]
    pub particles_file: String,

    /// Total simulation steps
    #[arg(long, default_value_t = 10000)]
    pub steps: u32,

    /// Size of time step
    #[arg(long, default_value_t = 0.001)]
    pub step_size: f64,

    /// Precision to use
    #[arg(long, default_value_t = 5)]
    pub precision: u8,

    /// Total particles to use
    #[arg(long, default_value_t = 1)]
    pub length: u32,

    /// Mode to run
    #[arg(long, default_value_t = 1)]
    pub mode: u8,

    /// Magnetic profile
    #[arg(long, default_value_t = 0)]
    pub magprof: u8,

    /// Total points
    #[arg(long, default_value_t = usize::MAX)]
    pub num_particles: usize,

    /// Phi angle
    #[arg(long, default_value_t = 0)]
    pub phi_angle: u32,

    /// Dimension
    #[arg(long, default_value_t = 1)]
    pub dimension: u8,

    /// Output directory
    #[arg(short, long)]
    pub output: String,

    /// How often to write output files
    #[arg(short, default_value_t = 10)]
    pub write_frequency: u32,
}
