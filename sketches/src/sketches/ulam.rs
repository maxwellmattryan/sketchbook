#![allow(unused)]

use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

pub fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .run();
}

struct Model {}

/// Initializes the app state (e.g. window, GUI) and performs startup
/// tasks like loading images or other assets.
fn model(app: &App) -> Model {
    Model {}
}

/// Updates the state of the model (hence the `&mut`), running at a constant time interval.
fn update(app: &App, model: &mut Model, update: Update) {}

/// Presents the state of the model (hence no `&mut`) to a window via the `Frame` object.
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(ANTIQUEWHITE);
}
