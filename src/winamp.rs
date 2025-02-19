use crate::player::AudioPlayerSupport;

pub struct Winamp;

impl AudioPlayerSupport for Winamp {
    fn play_mp3a(&self, song: &str) {
        println!("Winamp -> Playing mp3a: {}", song);
    }

    fn play_mp4a(&self, song: &str) {
        println!("Winamp -> Playing mp4a: {}", song);
    }

    fn play_mp4(&self, song: &str) {
        println!("Winamp -> Playing mp4: {}", song);
    }
}
