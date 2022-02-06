use crate::{
    commands::{ExportCommand, ViewCommand},
    error::SketchbookResult,
};

use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn run(&self) -> SketchbookResult<()>;
}

/// Easy management of your generative art sketches.
#[derive(structopt::StructOpt)]
#[structopt(
    author = "Matthew Maxwell <maxwellmattryan@gmail.com>",
    version = env!("CARGO_PKG_VERSION"),
)]
pub enum Cli {
    Export(ExportCommand),
    View(ViewCommand),
}

#[async_trait]
impl Command for Cli {
    async fn run(&self) -> SketchbookResult<()> {
        match self {
            Self::Export(cmd) => cmd.run().await,
            Self::View(cmd) => cmd.run().await,
        }
    }
}
