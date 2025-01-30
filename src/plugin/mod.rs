use std::{
    fs::read_dir,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    thread,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginManagerError {
    #[error("Failed to load plugin")]
    LoadPluginError,
}

#[derive(Default)]
pub struct PluginManager {
    plugins: Vec<Child>,
    stdin: Vec<Option<BufWriter<ChildStdin>>>,
}

impl PluginManager {
    pub fn load_file(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        let mut plugin = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| PluginManagerError::LoadPluginError)?;

        let stdout = plugin
            .stdout
            .take()
            .ok_or(PluginManagerError::LoadPluginError)?;

        let _ = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut buf = String::new();
                if let Err(_) = reader.read_to_string(&mut buf) {
                    continue;
                }
            }
        });

        let stdin = plugin
            .stdin
            .take()
            .ok_or(PluginManagerError::LoadPluginError)?;

        self.stdin.push(Some(BufWriter::new(stdin)));

        self.plugins.push(plugin);

        Ok(())
    }

    pub fn load_dir(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        for entry in read_dir(path).map_err(|_| PluginManagerError::LoadPluginError)? {
            self.load_file(
                entry
                    .map_err(|_| PluginManagerError::LoadPluginError)?
                    .path(),
            )?;
        }

        Ok(())
    }

    pub fn write(&mut self, index: usize, buf: &str) -> Result<(), PluginManagerError> {
        if let Some(stdin) = &mut self.stdin[index] {
            stdin
                .write(buf.as_bytes())
                .map_err(|_| PluginManagerError::LoadPluginError)?;
        }

        Ok(())
    }
}
