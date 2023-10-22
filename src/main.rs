mod models;
use core::fmt;
use models::{*, nannou_model::NannouModel};
use clap::{Parser, ValueEnum};

fn main() {
    let args = CliArgs::parse();
    // let model = get_model(args.model_to_run);
    // model.run_model();
    match args.model_to_run {
        Model::SinWave => sin_wave::SinWave.run_model(),
        Model::MovingCircle => moving_circle::MovingCircle.run_model(),
        Model::MovingSinWave => moving_sin_wave::MovingSinWave.run_model(),
    }

}

#[derive(Parser, Clone)]
struct CliArgs {
    #[clap(short, long, value_enum)]
    model_to_run: Model
}

#[derive(ValueEnum, Clone)]
enum Model {
    SinWave,
    MovingCircle,
    MovingSinWave
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
