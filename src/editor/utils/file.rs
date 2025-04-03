use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::anyhow;

#[derive(Default)]
pub struct EditorFile {
    path: Option<PathBuf>,
    file: Option<File>,
}

impl EditorFile {
    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        let file = if path.is_file() {
            Some(Self::open_file(&path)?)
        } else {
            None
        };

        Ok(Self {
            path: Some(path),
            file,
        })
    }

    pub fn _set_file_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn read(&mut self) -> anyhow::Result<String> {
        let Some(_) = &self.path else {
            return Err(anyhow!("File name is missing."));
        };

        Ok(match &mut self.file {
            None => String::from("\n"),
            Some(file) => {
                file.seek(SeekFrom::Start(0))?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                if buf.ends_with("\n") {
                    buf.push('\n');
                }
                buf
            }
        })
    }

    pub fn write(&mut self, content: &str) -> anyhow::Result<()> {
        let Some(path) = &self.path else {
            return Err(anyhow!("File name is missing."));
        };

        if self.file.is_none() {
            self.file = Some(Self::open_file(path)?);
        }

        match &mut self.file {
            None => panic!(),
            Some(file) => {
                file.seek(SeekFrom::Start(0))?;
                file.write_all(content.as_bytes())?;
            }
        }

        Ok(())
    }

    fn open_file(path: &PathBuf) -> anyhow::Result<File> {
        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?)
    }
}
