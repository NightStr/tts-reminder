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
    /// Частота повторений в секундах
    #[arg(short, long, default_value_t = 30)]
    repeat: u64,

    /// Сообщение
    #[arg(short, long)]
    message: String,
}


fn main() {
    let args = Args::parse();
    dbg!("Параметры {:?}", &args);

    let player_duration = Duration::new(5, 0);
    let sleep_duration = Duration::new(args.repeat, 0) - player_duration;

    let file_repository = CachedVoicerssFileRepository::new();
    let player = PlayFromFile::new(&file_repository, Duration::from_secs(5));
    let repeater = repeaters::SimpleRepeater::new(
        args.message,
        &player
    );
    if let Err(e) = repeater.repeat(sleep_duration) {
        eprintln!("{}", e);
    }
}
