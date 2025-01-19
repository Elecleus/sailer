pub struct Music<'a> {
    pub id: &'a str,
    pub title: Option<&'a str>,
    pub artist: Option<&'a str>,
    pub album: Option<&'a str>,
}

impl Music<'_> {
    pub fn new<'a>(
        id: &'a str,
        title: Option<&'a str>,
        artist: Option<&'a str>,
        album: Option<&'a str>,
    ) -> Music<'a> {
        Music {
            id,
            title,
            artist,
            album,
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> &str {
        self.id
    }
    #[allow(dead_code)]
    pub fn get_title(&self) -> &str {
        self.title.unwrap_or("Unknown")
    }
    #[allow(dead_code)]
    pub fn get_artist(&self) -> &str {
        self.artist.unwrap_or("Unknown")
    }
    #[allow(dead_code)]
    pub fn get_album(&self) -> &str {
        self.album.unwrap_or("Unknown")
    }
    // pub fn set_title(&mut self, title: Option<&str>) {
    //     self.title = title;
    // }
    // pub fn set_artist(&mut self, artist: Option<&str>) {
    //     self.artist = artist;
    // }
    // pub fn set_album(&mut self, album: Option<&str>) {
    //     self.album = album;
    // }
    pub fn to_string(&self) -> String {
        format!(
            "{}: {} - {} ({})",
            self.id,
            self.artist.unwrap_or("Unknown"),
            self.title.unwrap_or("Unknown"),
            self.album.unwrap_or("Unknown")
        )
    }
    pub fn to_string_no_id(&self) -> String {
        format!(
            "{} - {}",
            self.artist.unwrap_or("Unknown"),
            self.title.unwrap_or("Unknown")
        )
    }
}
