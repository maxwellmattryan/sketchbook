use nannou::prelude::*;

pub(crate) fn get_capture_frame_path(name: &str, app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("../exports")
        .join(name)
        .join(format!("{:03}", frame.nth()))
        .with_extension("png")
}
