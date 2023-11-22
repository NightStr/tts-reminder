use anyhow::Result;
use crate::repositories::FileRepository;

use std::io::{BufReader};
use rodio::{OutputStream};

pub trait Player {
    fn play(&self, text: &str) -> Result<()>;
}


pub struct PlayFromFile<T: FileRepository> {
    volume: f32,
    file_repository: T
}

impl<T: FileRepository> PlayFromFile<T> {
    pub fn new(volume: f32, file_repository: T) -> Self {
        PlayFromFile{volume, file_repository}
    }
}

impl<T: FileRepository> Player for PlayFromFile<T> {
    fn play(&self, text: &str) -> Result<()> {
        let reader = BufReader::new(self.file_repository.get_file(text)?);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = stream_handle.play_once(reader).unwrap();
        sink.set_volume(self.volume);
        sink.sleep_until_end();
        Ok(())
    }
}
