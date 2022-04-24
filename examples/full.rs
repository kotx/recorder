#[allow(dead_code)]
mod song {
    use recorder::record;

    #[record] // try commenting this out! it'll break visibility
    pub struct Song {
        name: String,
        artist: Artist,
        #[record(skip)] // this field's visibility is preserved
        pub(crate) similar: Vec<Self>,
    }

    impl Song {
        pub fn new<S: Into<String>>(name: S, artist: S) -> Self {
            Self {
                name: name.into(),
                artist: Artist(artist.into()),
                similar: vec![],
            }
        }
    }

    // Visiblity is preserved for #[record(skip)]
    pub fn get_similar(song: Song) -> Vec<Song> {
        song.similar
    }

    #[record]
    pub struct Artist(String);
}

fn main() {
    let song = song::Song::new("Lagtrain", "inabakumori");

    println!("Song name: {}", song.name);
    println!("Artist: {}", song.artist.0);
    // println!("Similar (you shouldn't see this, it should error if uncommented): {:?}", song.similar);
    println!("Similar: {:?}", song::get_similar(song));
}
