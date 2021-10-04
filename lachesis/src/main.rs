mod app;

use crate::app::{Arguments, Subcommand};

use anyhow::Result;
use klotho::storage::{LocalFilesystem, Storage};
use log::info;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env()
        .expect("Invalid logger setting")
        .start()
        .expect("Failed to start logger");

    let args = Arguments::from_args();

    match args.subcommand {
        Subcommand::Register { .. } => {
            let storage = LocalFilesystem::new("./data").await?;
            let filename = storage.store(b"Hello", Some(".txt")).await?;
            info!("Stored into {}", filename);
        }
        Subcommand::Update { .. } => todo!(),
        Subcommand::Delete { .. } => todo!(),
        Subcommand::Fetch { .. } => todo!(),
    }

    Ok(())
}
