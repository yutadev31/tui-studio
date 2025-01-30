use std::{
    fs::read_dir,
    io,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginManagerError {
    #[error("{0}")]
    IOError(#[from] io::Error),
}

#[derive(Default)]
pub struct PluginManager {
    plugins: Vec<Child>,
}

impl PluginManager {
    pub fn load_file(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        self.plugins.push(
            Command::new(path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?,
        );

        Ok(())
    }

    pub fn load_dir(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        for entry in read_dir(path)? {
            self.load_file(entry?.path())?;
        }

        Ok(())
    }
}
