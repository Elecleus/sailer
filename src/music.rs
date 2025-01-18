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
    pub fn get_title(&self) -> Option<&str> {
        self.title
    }
    #[allow(dead_code)]
    pub fn get_artist(&self) -> Option<&str> {
        self.artist
    }
    #[allow(dead_code)]
    pub fn get_album(&self) -> Option<&str> {
        self.album
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
            "{}: {} by {} ({})",
            self.id,
            self.title.unwrap_or("Unknown"),
            self.artist.unwrap_or("Unknown"),
            self.album.unwrap_or("Unknown")
        )
    }
}
