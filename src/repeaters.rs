use anyhow::Result;

use std::thread::sleep;
use std::time::Duration;
use crate::players::Player;

pub trait Repeater {
    fn repeat(&self, duration: Duration) -> Result<()>;
}


pub struct SimpleRepeater<'a> {
    text: String,
    player: &'a dyn Player
}

impl<'a> SimpleRepeater<'a> {
    pub fn new(text: String, player: &'a dyn Player) -> Self {
        Self{text, player}
    }
}

impl<'a> Repeater for SimpleRepeater<'a> {
    fn repeat(&self, duration: Duration) -> Result<()> {
        loop {
            self.player.play(self.text.as_str())?;
            sleep(duration);
        }
    }
}
