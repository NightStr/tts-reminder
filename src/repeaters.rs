use anyhow::Result;

use std::thread::sleep;
use std::time::Duration;
use crate::players::Player;

pub trait Repeater {
    fn repeat(&self, duration: Duration) -> Result<()>;
}


pub struct SimpleRepeater<T: Player> {
    text: String,
    player: T
}

impl<T: Player> SimpleRepeater<T> {
    pub fn new(text: String, player: T) -> Self {
        Self{text, player}
    }
}

impl<T: Player> Repeater for SimpleRepeater<T> {
    fn repeat(&self, duration: Duration) -> Result<()> {
        loop {
            self.player.play(self.text.as_str())?;
            sleep(duration);
        }
    }
}
