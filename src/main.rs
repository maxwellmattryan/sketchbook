mod sketches;
mod templates;

use sketchbook::{
    cli::{Cli, Command},
    error::{SketchbookError, SketchbookResult},
};

use structopt::StructOpt;

#[tokio::main]
async fn main() -> SketchbookResult<()> {
    match Cli::from_args().run().await {
        Ok(_) => Ok(()),
        Err(_) => panic!("{:?}", SketchbookError::CannotRunCommand),
    }
}
