use crate::repositories::FileRepository;

use std::io::{BufReader};
use rodio::{OutputStream};

pub trait Player {
    fn play(&self, text: &str) -> Result<(), &str>;
}


pub struct PlayFromFile<'a> {
    volume: f32,
    file_repository: &'a dyn FileRepository
}

impl<'a> PlayFromFile<'a> {
    pub fn new(volume: f32, file_repository: &'a dyn FileRepository) -> Self {
        return PlayFromFile{volume, file_repository}
    }
}

impl<'a> Player for PlayFromFile<'a> {
    fn play(&self, text: &str) -> Result<(), &str> {
        let reader = BufReader::new(self.file_repository.get_file(text)?);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = stream_handle.play_once(reader).unwrap();
        sink.set_volume(self.volume);
        sink.sleep_until_end();
        return Ok(());
    }
}
