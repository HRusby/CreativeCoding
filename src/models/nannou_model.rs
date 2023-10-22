use nannou::prelude::*;
pub trait NannouModel{
    type Model;
    // Invoke the running of the model
    fn run_model(&self);
    // Run once at the begining of the app, produces a fresh instance of Model
    fn model(app: &App) -> Self::Model;
    // General type of event occurring on a time frequency (60 times per second) 
    fn update(_app: &App, _model: &mut Self::Model, _update: Update);
    // Presents the Model to some window
    fn view(app: &App, _model: &Self::Model, frame: Frame);

}
