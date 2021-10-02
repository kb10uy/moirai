//! Local filesystem implementation

use crate::storage::{Error, ErrorKind, Result, Storage};

use async_std::{
    fs::{remove_file, write},
    path::{Path, PathBuf},
};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct LocalFilesystem {
    base_dir: PathBuf,
}

impl LocalFilesystem {
    /// Checks whether specified directory exists and creates new instance.
    pub async fn new(base_dir: impl AsRef<Path>) -> Result<LocalFilesystem> {
        let base_dir: PathBuf = base_dir.as_ref().into();

        if !base_dir.is_dir().await {
            let inner = format!(
                "Path {} is not a directory or accessible",
                base_dir.display()
            );
            return Err(Error::new(
                ErrorKind::InvalidConfiguration,
                Some(inner.into()),
            ));
        }

        Ok(LocalFilesystem { base_dir })
    }
}

#[async_trait]
impl Storage for LocalFilesystem {
    async fn store(&self, data: &[u8], extension: Option<&str>) -> Result<String> {
        let mut filename = Uuid::new_v4().to_string();
        if let Some(ext) = extension {
            filename.push_str(ext);
        }

        let target_path = self.base_dir.join(&filename);
        match write(&target_path, data).await {
            Ok(()) => Ok(filename),
            Err(e) => Err(Error::new(ErrorKind::CannotWrite, Some(e.into()))),
        }
    }

    async fn path(&self, filename: &str) -> Result<String> {
        // TODO: Give another form
        let filename = Path::new(filename)
            .file_name()
            .ok_or_else(|| Error::new(ErrorKind::CannotWrite, None))?;

        Ok(self.base_dir.join(filename).to_string_lossy().into_owned())
    }

    async fn remove(&self, filename: &str) -> Result<()> {
        let filename = Path::new(filename)
            .file_name()
            .ok_or_else(|| Error::new(ErrorKind::CannotWrite, None))?;

        match remove_file(self.base_dir.join(filename)).await {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::new(ErrorKind::CannotWrite, Some(e.into()))),
        }
    }
}
