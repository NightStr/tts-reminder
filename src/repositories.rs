use anyhow::{anyhow, Result};

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::copy;
use std::path::Path;

pub trait FileRepository {
    fn get_file(&self, text: &str) -> Result<File>;
}


pub struct CachedVoicerssFileRepository{
    app_key: String
}

impl CachedVoicerssFileRepository {
    pub fn new(app_key: String) -> Self {
        CachedVoicerssFileRepository{app_key}
    }

    fn generate_file(&self, text: &str, filename: &String) -> Result<()> {
        let response = reqwest::blocking::get(
            format!(
                "https://api.voicerss.org/?key={}&hl=ru-ru&v=Marina&src={}",
                self.app_key,
                text
            )
        ).unwrap();
        let content_type = response.headers().get("Content-Type")
            .expect("Content-Type not found")
            .to_str()
            .expect("Content-Type is not ASCII");
        if response.status().is_success() && content_type == "audio/wav" {
            if let Some(parent_dir) = Path::new(&filename).parent() {
                if !parent_dir.exists() {
                    std::fs::create_dir_all(parent_dir)?
                }
            }
            let mut file = File::create(filename).unwrap();

            // Копируем содержимое ответа в файл
            copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();

            dbg!("Файл успешно загружен и сохранен: {}", filename);
        } else {
            // В случае неудачного ответа, выводим сообщение об ошибке
            return Err(anyhow!(format!(
                "Не удалось загрузить файл. Ответ сервиса: {}", response.text().expect(
                    "Неизвестная ошибка"
                )
            )));
        }
        Ok(())
    }
}

impl FileRepository for CachedVoicerssFileRepository {
    fn get_file(&self, text: &str) -> Result<File> {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        let filename = format!("resources/cache/{}.wav", hash);
        let file_path = Path::new(&filename);
        if !file_path.exists() {
            dbg!("Генерируем файл");
            self.generate_file(text, &filename)?;
        } else {
            dbg!("Берем файл из кэша");
        }
        Ok(File::open(&filename)?)
    }
}
