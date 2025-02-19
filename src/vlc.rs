use crate::player::AudioPlayerSupport;

pub struct Vlc;

impl AudioPlayerSupport for Vlc {
    fn play_mp3a(&self, song: &str) {
        println!("Vlc -> Playing mp3a: {}", song);
    }

    fn play_mp4a(&self, song: &str) {
        println!("Vlc -> Playing mp4a: {}", song);
    }

    fn play_mp4(&self, song: &str) {
        println!("Vlc -> Playing mp4: {}", song);
    }
}
