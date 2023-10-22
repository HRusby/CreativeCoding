mod models;
use core::fmt;

use models::{*, nannou_model::NannouModel};
use clap::Parser;
use clap::ValueEnum;
fn main() {
    let args = CliArgs::parse();
    let model = get_model(args.model_to_run);
    model.run_model();

}

fn get_model(model: Model) -> impl NannouModel{
    match model{
        Model::SinWave => return sin_wave::SinWave,
    }
}

#[derive(Parser, Clone)]
struct CliArgs {
    #[clap(short, long, value_enum)]
    model_to_run: Model
}

#[derive(ValueEnum, Clone)]
enum Model {
    SinWave
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
