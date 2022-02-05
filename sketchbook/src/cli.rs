use crate::{commands::ViewCommand, error::SketchbookResult};

use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn run(&self) -> SketchbookResult<()>;
}

#[derive(structopt::StructOpt)]
#[structopt(
    author = "Matthew Maxwell <maxwellmattryan@gmail.com>",
    version = env!("CARGO_PKG_VERSION"),
)]
pub enum Cli {
    View(ViewCommand),
}

#[async_trait]
impl Command for Cli {
    async fn run(&self) -> SketchbookResult<()> {
        match self {
            Self::View(cmd) => cmd.run().await,
        }
    }
}
