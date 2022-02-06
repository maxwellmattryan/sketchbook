use crate::{
    cli::{Command},
    error::{SketchbookResult},
    parsing::{try_sketch_name_from_str},
};

use async_trait::async_trait;

/// Render a sketch from within your sketchbook.
#[derive(structopt::StructOpt)]
pub struct ExportCommand {
    /// The particular sketch to render (e.g. "my_sketch" or "MySketch")
    #[structopt(parse(try_from_str = try_sketch_name_from_str))]
    pub sketch_name: String
}

#[async_trait]
impl Command for ExportCommand {
    async fn run(&self) -> SketchbookResult<()> {
        Ok(())
    }
}
