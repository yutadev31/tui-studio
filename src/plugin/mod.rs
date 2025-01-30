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
        debug!("{} plugin is starting to load.", path.to_str().unwrap());

        let mut plugin = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| PluginManagerError::LoadPluginFailed(err))?;

        let stdout = plugin
            .stdout
            .take()
            .ok_or(PluginManagerError::GetStdoutFailed)?;

        let _handle = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut buf = String::new();
                if let Err(_) = reader.read_line(&mut buf) {
                    continue;
                }

                debug!("{}", buf);
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
        for entry in read_dir(path)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    error!("{}", err);
                    continue;
                }
            };

            self.load_file(entry.path())?;
        }

        Ok(())
    }

    pub fn write(&mut self, index: usize, buf: &str) -> Result<(), PluginManagerError> {
        if let Some(stdin) = &mut self.stdin.get_mut(index).and_then(Option::as_mut) {
            stdin.write(buf.as_bytes())?;
            stdin.flush()?;
            Ok(())
        } else {
            Err(PluginManagerError::GetStdinFailed)
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        for plugin in &mut self.plugins {
            if let Err(e) = plugin.kill() {
                error!("Failed to kill plugin: {}", e);
            }
        }
    }
}
