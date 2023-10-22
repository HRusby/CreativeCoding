mod models;
use models::{*, nannou_model::NannouModel};
fn main() {
    let sin_wave = sin_wave::SinWave{}; 
    sin_wave.run_model();
}

