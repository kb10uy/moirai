use anyhow::Result;
use klotho::storage::{LocalFilesystem, Storage};

#[async_std::main]
async fn main() -> Result<()> {
    let storage = LocalFilesystem::new("./data").await?;
    let filename = storage.store(b"Hello", Some(".txt")).await?;
    println!("Stored into {}", filename);
    Ok(())
}
