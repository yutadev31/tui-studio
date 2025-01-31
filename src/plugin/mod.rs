use std::{
    fs::read_dir,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    thread,
};

use log::{debug, error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginManagerError {
    #[error("Failed to load plugin")]
    LoadPluginFailed(#[source] io::Error),

    #[error("Failed to get stdout")]
    GetStdoutFailed,

    #[error("Failed to get stdin")]
    GetStdinFailed,

    #[error("Failed to read directory")]
    ReadDirFailed(#[source] io::Error),

    #[error("{0}")]
    IOError(#[from] io::Error),
}

#[derive(Default)]
pub struct PluginManager {
    plugins: Vec<Child>,
    stdin: Vec<Option<BufWriter<ChildStdin>>>,
}

impl PluginManager {
    pub fn load_file(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        debug!("{} plugin is starting to load.", path.display());

        let mut plugin = Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| PluginManagerError::LoadPluginFailed(err))?;

        let stdout = plugin
            .stdout
            .take()
            .ok_or(PluginManagerError::GetStdoutFailed)?;

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut buf = String::new();
                match reader.read_line(&mut buf) {
                    Ok(_) => {
                        debug!("{}", buf);
                    }
                    Err(e) => {
                        error!("Error reading plugin stdout: {}", e);
                        break;
                    }
                }
            }
        });

        let stdin = plugin
            .stdin
            .take()
            .ok_or(PluginManagerError::GetStdinFailed)?;

        self.stdin.push(Some(BufWriter::new(stdin)));
        self.plugins.push(plugin);

        debug!("plugin is loaded.");
        Ok(())
    }

    pub fn load_dir(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        read_dir(&path)
            .map_err(|err| PluginManagerError::ReadDirFailed(err))?
            .filter_map(Result::ok)
            .for_each(|entry| {
                if let Err(e) = self.load_file(entry.path()) {
                    error!(
                        "Failed to load plugin from {}: {:?}",
                        entry.path().display(),
                        e
                    );
                }
            });

        Ok(())
    }

    pub fn write(&mut self, index: usize, buf: &str) -> Result<(), PluginManagerError> {
        if let Some(stdin) = self.stdin.get_mut(index).and_then(Option::as_mut) {
            stdin.write(buf.as_bytes())?;
            stdin.flush()?;
            Ok(())
        } else {
            Err(PluginManagerError::GetStdinFailed)
        }
    }

    pub fn kill(&mut self) {
        for plugin in &mut self.plugins {
            if let Err(e) = plugin.kill() {
                error!("Failed to kill plugin: {}", e);
            }
        }
    }
}
