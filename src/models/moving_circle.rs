use super::nannou_model::NannouModel;

pub struct MovingCircle;
pub struct MovingCircleModel;
impl NannouModel for MovingCircle{
    type Model = MovingCircleModel;

    fn run_model(&self) {
        todo!()
    }

    fn model(app: &nannou::App) -> Self::Model {
        todo!()
    }

    fn update(_app: &nannou::App, _model: &mut Self::Model, _update: nannou::prelude::Update) {
        todo!()
    }

    fn view(app: &nannou::App, _model: &Self::Model, frame: nannou::Frame) {
        todo!()
    }
}
