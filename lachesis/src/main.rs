mod app;

use anyhow::Result;
use klotho::storage::{LocalFilesystem, Storage};
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env()
        .expect("Invalid logger setting")
        .start()
        .expect("Failed to start logger");

    let args = app::Arguments::from_args();

    let storage = LocalFilesystem::new("./data").await?;
    let filename = storage.store(b"Hello", Some(".txt")).await?;
    println!("Stored into {}", filename);
    Ok(())
}
