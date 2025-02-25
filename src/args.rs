use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to resource folder
    #[arg(long)]
    pub(crate) resource_path: String,

    /// Particles file
    #[arg(long)]
    pub(crate) particles_file: String,

    /// Total simulation steps
    #[arg(long, default_value_t = 10000)]
    pub(crate) steps: u32,

    /// Size of time step
    #[arg(long, default_value_t = 0.001)]
    pub(crate) step_size: f64,

    /// Precision to use
    #[arg(long, default_value_t = 5)]
    pub(crate) precision: u8,

    /// Total particles to use
    #[arg(long, default_value_t = 1)]
    pub(crate) length: u32,

    /// Mode to run
    #[arg(long, default_value_t = 1)]
    pub(crate) mode: u8,

    /// Magnetic profile
    #[arg(long, default_value_t = 0)]
    pub(crate) magprof: u8,

    /// Total points
    #[arg(long, default_value_t = 10000)]
    pub(crate) num_points: u32,

    /// Phi angle
    #[arg(long, default_value_t = 0)]
    pub(crate) phi_angle: u32,

    /// Dimension
    #[arg(long, default_value_t = 1)]
    pub(crate) dimension: u8,

    /// Output directory
    #[arg(long)]
    pub(crate) output: String,
}
