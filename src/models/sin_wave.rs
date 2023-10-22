use nannou::prelude::*;
use super::nannou_model::NannouModel;

pub struct SinWave;

impl NannouModel for SinWave{
    type Model = SinWaveModel;
    fn run_model(&self) {
        nannou::app(Self::model)
            .update(Self::update)
            .run();
    }

    fn model(app: &App) -> Self::Model{
        let _window = app.new_window().view(Self::view).build().unwrap();
        SinWaveModel {_window}
    }
    
    fn update(_app: &App, _model: &mut Self::Model, _update: Update) {}

    fn view(app: &App, _model: &Self::Model, frame: Frame) {
        let draw = app.draw();
        // let window_size = app.window(model.window).unwrap().rect().w_h();
        // let quarter_width = window_size.0 * 0.25;
        // let quarter_height = window_size.1 * 0.25;
        draw.background().color(BLACK);
        let points = (0..10000).map(|i|{
            let x = (i as f32 - 500.0)*0.01;
            let point = pt2(x, x.sin()) * 50.0;
            (point, WHITE)
        });
        draw.polyline()
            .weight(1.0)
            .points_colored(points);
        draw.to_frame(app, &frame).unwrap();
    }

}

pub struct SinWaveModel {
    _window: window::Id,
}
