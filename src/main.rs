mod player;
mod vlc;
mod winamp;

use player::{AudioPlayer, MediaPlayer};
use vlc::Vlc;
use winamp::Winamp;

fn main() {
    let vlc_player = AudioPlayer::new(Box::new(Vlc));
    let winnap_player = AudioPlayer::new(Box::new(Winamp));

    vlc_player.play("song.mp3a");
    vlc_player.play("song.mp4a");
    vlc_player.play("song.mp4");
    vlc_player.play("song.mp3");

    winnap_player.play("song.mp3a");
    winnap_player.play("song.mp4a");
    winnap_player.play("song.mp4");
    winnap_player.play("song.mp3");
}
