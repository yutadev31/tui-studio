use std::{fs, io, path::PathBuf};

use libloading::Library;
use thiserror::Error;

use super::Plugin;

#[derive(Debug, Error)]
pub enum PluginManagerError {
    #[error("{0}")]
    LibraryLoadingError(#[from] libloading::Error),

    #[error("{0}")]
    IOError(#[from] io::Error),
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }

    pub fn load(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "so" || ext == "dylib" || ext == "dll" {
                    self.load_lib(path)?;
                }
            }
        }
        Ok(())
    }

    fn load_lib(&mut self, path: PathBuf) -> Result<(), PluginManagerError> {
        let lib = unsafe { Library::new(path)? };

        let plugin = unsafe {
            let get_plugin: libloading::Symbol<extern "C" fn() -> Box<dyn Plugin>> =
                lib.get(b"load_plugin")?;

            get_plugin()
        };

        self.libraries.push(lib);
        self.plugins.push(plugin);

        Ok(())
    }

    pub fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }
}
