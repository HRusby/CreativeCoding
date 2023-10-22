pub use sin_wave::run_model;
mod sin_wave{
    use nannou::prelude::*;
    pub fn run_model() {
        nannou::app(model)
            .update(update)
            .run();
    }

    struct SinWaveModel {
        _window: window::Id,
    }

    // Run once at the begining of the app, produces a fresh instance of Model
    fn model(app: &App) -> SinWaveModel {
        let _window = app.new_window().view(view).build().unwrap();
        SinWaveModel {_window}
    }

    // General type of event occurring on a time frequency (60 times per second) 
    fn update(_app: &App, _model: &mut SinWaveModel, _update: Update) {}

    // Presents the Model to some window
    fn view(app: &App, _model: &SinWaveModel, frame: Frame) {
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

