use crate::{
    cli::Command,
    error::{SketchbookError, SketchbookResult},
    sketch::{Sketch, SketchEntrypoint},
};

use async_trait::async_trait;
use regex::Regex;

fn try_sketch_name_from_str(arg: &str) -> SketchbookResult<String> {
    let re = Regex::new(r"[^-_.a-zA-Z0-9]").unwrap();
    if re.is_match(arg) {
        Err(SketchbookError::InvalidSketchName(arg.to_string()))
    } else {
        Ok(arg.to_string())
    }
}

#[derive(structopt::StructOpt)]
pub struct ViewCommand {
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
