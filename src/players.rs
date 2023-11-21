use crate::repositories::FileRepository;

use std::io::{BufReader};
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source};

pub trait Player {
    fn play(&self, text: &str) -> Result<(), &str>;
}


pub struct PlayFromFile<'a> {
    sleep_duration: Duration,
    file_repository: &'a dyn FileRepository
}

impl<'a> PlayFromFile<'a> {
    pub fn new(file_repository: &'a dyn FileRepository, sleep_duration: Duration) -> Self {
        return PlayFromFile{sleep_duration, file_repository}
    }
}

impl<'a> Player for PlayFromFile<'a> {
    fn play(&self, text: &str) -> Result<(), &str> {
        let reader = BufReader::new(self.file_repository.get_file(text)?);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Создаем декодер для файла
        let source = Decoder::new(reader).expect("Failed to create decoder");

        stream_handle.play_raw(source.convert_samples()).unwrap();
        // Даем время для воспроизведения файла
        thread::sleep(self.sleep_duration);
        return Ok(());
    }
}
