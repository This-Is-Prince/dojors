#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub genre: Genre,
    pages: u32,
    pages_read: u32,
}

impl Book {
    pub fn new(t: &str, g: Genre, p: u32) -> Self {
        Self {
            title: t.into(),
            genre: g,
            pages: p,
            pages_read:0,
        }
    }

    pub fn read_pages(&mut self, n: u32) {
        if self.pages_read + n <= self.pages {
            self.pages_read += n;
        }
    }
}

pub struct Marker;
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy)]
pub enum Genre {
    Fiction,
    Tech,
    Comics,
}