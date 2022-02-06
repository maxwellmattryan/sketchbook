use crate::{
    cli::Command,
    error::SketchbookResult,
    parsing::try_sketch_name_from_str,
    sketch::{Sketch, SketchEntrypoint},
};

use async_trait::async_trait;

/// View a sketch from within your sketchbook.
#[derive(structopt::StructOpt)]
pub struct ViewCommand {
    /// The particular sketch to view (e.g. "my_sketch" or "MySketch").
    #[structopt(parse(try_from_str = try_sketch_name_from_str))]
    pub sketch_name: String,
}

#[async_trait]
impl Command for ViewCommand {
    async fn run(&self) -> SketchbookResult<()> {
        let sketch: Sketch = self.sketch_name.parse().unwrap();
        let entrypoint: SketchEntrypoint = sketch.to_entrypoint();

        Ok(entrypoint())
    }
}
