pub mod book_list;
pub use book_list::*;
pub mod info;
pub use info::*;
pub mod chapter_list;
pub use chapter_list::*;

pub struct Book {
    pub book_id: String,
    pub author: String,
    pub cover_url: Option<String>,
    pub intro: String,
    pub kind: String,
    pub last_chapter: String,
    pub name: String,
    pub word_count: String,
    pub chapter_list: Vec<Chapter>,
    pub current_chapter: usize,
    pub line_percent: f64,
}
