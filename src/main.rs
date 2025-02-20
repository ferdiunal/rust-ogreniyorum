trait MediaPlayer {
    fn play(&self, file_name: &str);
}

struct AudioPlayer;
struct VideoPlayer;

impl MediaPlayer for AudioPlayer {
    fn play(&self, file_name: &str) {
        println!("Playing audio file: {}", file_name);
    }
}

impl MediaPlayer for VideoPlayer {
    fn play(&self, file_name: &str) {
        println!("Playing video file: {}", file_name);
    }
}

fn media_player_factory(media_type: &str) -> Box<dyn MediaPlayer> {
    match media_type {
        "audio" => Box::new(AudioPlayer),
        "video" => Box::new(VideoPlayer),
        _ => panic!("Invalid media type"),
    }
}

fn main() {
    let audio_player = media_player_factory("audio");
    audio_player.play("song.mp3");

    let video_player = media_player_factory("video");
    video_player.play("movie.mp4");
}
