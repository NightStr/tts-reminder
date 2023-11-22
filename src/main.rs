mod repositories;
mod players;
mod repeaters;

use std::time::Duration;
use repositories::{CachedVoicerssFileRepository};
use players::{PlayFromFile};
use repeaters::Repeater;
use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Громкость 1.0 - стандартнаая громкость. Чем выше число тем выше громкость
    #[arg(short, long, default_value_t = 1.0)]
    volume: f32,

    /// Частота повторений в секундах
    #[arg(short, long, default_value_t = 30)]
    repeat: u64,

    /// Сообщение
    #[arg(short, long)]
    message: String,

    /// Ключ для voicerss
    #[arg(short, long)]
    app_key: String,
}


fn main() {
    let args = Args::parse();
    dbg!("Параметры {:?}", &args);
    let repeater = repeaters::SimpleRepeater::new(
        args.message,
        PlayFromFile::new(
            args.volume,
            CachedVoicerssFileRepository::new(args.app_key)
        )
    );
    if let Err(e) = repeater.repeat(Duration::from_secs(args.repeat)) {
        eprintln!("{}", e);
    }
}
