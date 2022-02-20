#![allow(unused)]

use nannou::prelude::*;

use rand::{thread_rng, Rng};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

pub fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .run();
}

const NUM_COLUMNS: usize = 32;
const NUM_ROWS: usize = 24;
const PROBABILITY_THRESHOLD: f32 = 0.5;
const LINES_PER_FRAME: usize = NUM_COLUMNS / 2;

type Line = (Vec2, Vec2);

struct Model {
    current_index: usize,
    current_position: Vec2,
    lines: Vec<Line>,
}

/// Initializes the app state (e.g. window, GUI) and performs startup
/// tasks like loading images or other assets.
fn model(app: &App) -> Model {
    let window = app.window_rect();

    Model {
        current_index: window.left() as usize,
        current_position: pt2(window.left(), window.top()),
        lines: vec![
            (
                pt2(window.left() * 2.0, window.top() * 2.0),
                pt2(window.left() * 2.0, window.top() * 2.0)
            );
            NUM_COLUMNS * NUM_ROWS
        ],
    }
}

/// Updates the state of the model (hence the `&mut`), running at a constant time interval.
fn update(app: &App, model: &mut Model, update: Update) {
    for _ in 0..=LINES_PER_FRAME {
        let window = app.window_rect();
        let (width, height) = window.w_h();
        let (cell_width, cell_height) = (width / NUM_COLUMNS as f32, height / NUM_ROWS as f32);

        let probability: f32 = thread_rng().gen();
        let new_line: Line = if probability < PROBABILITY_THRESHOLD {
            (
                pt2(model.current_position.x, model.current_position.y),
                pt2(
                    model.current_position.x + cell_width,
                    model.current_position.y + cell_height,
                ),
            )
        } else {
            (
                pt2(model.current_position.x + cell_width, model.current_position.y),
                pt2(model.current_position.x, model.current_position.y + cell_height),
            )
        };

        std::mem::replace(&mut model.lines[model.current_index], new_line);
        model.current_index += 1;
        if model.current_index >= NUM_COLUMNS * NUM_ROWS {
            model.current_index = 0;
        }

        model.current_position.x += cell_width;
        if model.current_position.x >= window.right() + cell_width {
            model.current_position.x = window.left();
            model.current_position.y -= cell_height;
            if model.current_position.y <= window.bottom() - cell_height {
                model.current_position.y = window.top();
            }
        }
    }
}

/// Presents the state of the model (hence no `&mut`) to a window via the `Frame` object.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(ANTIQUEWHITE);

    for line in &model.lines {
        draw.line().color(BLACK).start(line.0).end(line.1);
    }

    draw.to_frame(app, &frame);
}
