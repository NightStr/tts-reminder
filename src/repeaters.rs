use std::thread::sleep;
use std::time::Duration;
use crate::players::Player;

pub trait Repeater {
    fn repeat(&self, duration: Duration) -> Result<(), &str>;
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
    fn repeat(&self, duration: Duration) -> Result<(), &str> {
        loop {
            if let Err(e) = self.player.play(self.text.as_str()) {
                return Err(e)
            };
            sleep(duration);
        }
    }
}
