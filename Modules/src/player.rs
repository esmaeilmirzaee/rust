pub fn video_cast(name: &str) -> String {
    format!("Oscar winner is {}", name)
}

pub fn audio_playing(name: &str) {
    play(name)
}

fn play(name: &str) {
    println!("Playing {}", name);
}