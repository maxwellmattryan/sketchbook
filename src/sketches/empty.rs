use nannou::prelude::*;

pub fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

/// Initializes the app state (e.g. window, GUI) and performs startup
/// tasks like loading images or other assets.
fn model(_app: &App) -> Model {
    Model {}
}

/// Updates the state of the model (hence the `&mut`), running at a constant time interval.
fn update(_app: &App, _model: &mut Model, _update: Update) {}

/// Presents the state of the model (hence no `&mut`) to a window via the `Frame` object.
fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(ORANGE);
}
