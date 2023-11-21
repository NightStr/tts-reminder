use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::copy;
use std::path::Path;

pub trait FileRepository {
    fn get_file(&self, text: &str) -> Result<File, &str>;
}


pub struct CachedVoicerssFileRepository{
    app_key: String
}

impl CachedVoicerssFileRepository {
    pub fn new(app_key: String) -> Self {
        return CachedVoicerssFileRepository{app_key}
    }

    fn generate_file(&self, text: &str, filename: &String) -> Result<(), &'static str> {
        let response = reqwest::blocking::get(
            format!(
                "http://api.voicerss.org/?key={}&hl=ru-ru&v=Marina&src={}",
                self.app_key,
                text
            )
        ).unwrap();
        if response.status().is_success() {
            if let Some(parent_dir) = Path::new(&filename).parent() {
                if !parent_dir.exists() {
                    if let Err(err) = std::fs::create_dir_all(parent_dir) {
                        dbg!("Ошибка при создании директории {}", err);
                        return Err("Error creating directories");
                    }
                }
            }
            let mut file = File::create(&filename).unwrap();

            // Копируем содержимое ответа в файл
            copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();

            dbg!("Файл успешно загружен и сохранен: {}", filename);
        } else {
            // В случае неудачного ответа, выводим сообщение об ошибке
            dbg!("Не удалось загрузить файл. Статус ответа: {}", response.status());
            return Err("Не удалось загрузить файл");
        }
        Ok(())
    }
}

impl FileRepository for CachedVoicerssFileRepository {
    fn get_file(&self, text: &str) -> Result<File, &str> {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        let filename = format!("resources/cache/{}.wav", hash);
        let file_path = Path::new(&filename);
        if !file_path.exists() {
            dbg!("Генерируем файл");
            if let Err(err) = self.generate_file(text, &filename,) {
                return Err(err);
            }
        } else {
            dbg!("Берем файл из кэша");
        }
        return Ok(File::open(&filename).unwrap());
    }
}
