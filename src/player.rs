pub trait MediaPlayer {
    fn play(&self, song: &str);
}

pub trait AudioPlayerSupport {
    fn play_mp3a(&self, song: &str);
    fn play_mp4a(&self, song: &str);
    fn play_mp4(&self, song: &str);
}

pub struct AudioPlayer {
    player: Box<dyn AudioPlayerSupport>,
}

impl AudioPlayer {
    pub fn new(player: Box<dyn AudioPlayerSupport>) -> Self {
        Self { player }
    }
}

impl MediaPlayer for AudioPlayer {
    fn play(&self, song: &str) {
        if song.ends_with(".mp3a") {
            self.player.play_mp3a(song);
        } else if song.ends_with(".mp4a") {
            self.player.play_mp4a(song);
        } else if song.ends_with(".mp4") {
            self.player.play_mp4(song);
        } else {
            println!("Unsupported format: {}", song);
        }
    }
}
