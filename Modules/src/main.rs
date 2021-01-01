mod player;
fn main() {
    println!("Modules!");
    player::audio_playing("Esmaeil MIRZAEE");
    let winner = player::video_cast("Esmaeil MIRZAEE");
    println!("{}", winner);
    clean::perform_clean();
    clean::files::removing();
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning the theatre!");
    }

    pub mod files {
        pub fn removing() {
            println!("Clean unused files!");
        }
    }
}