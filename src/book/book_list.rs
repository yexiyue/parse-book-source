use std::ops::{Deref, DerefMut};

use crate::HttpClient;
use crate::Result;
use reqwest::Response;

#[derive(Debug, Clone)]
pub struct BookList {
    pub book_list: Vec<BookListItem>,
}

impl BookList {
    pub fn new(book_list: Vec<BookListItem>) -> Self {
        BookList { book_list }
    }
}

impl Deref for BookList {
    type Target = Vec<BookListItem>;

    fn deref(&self) -> &Self::Target {
        &self.book_list
    }
}

impl DerefMut for BookList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.book_list
    }
}

impl From<Vec<BookListItem>> for BookList {
    fn from(book_list: Vec<BookListItem>) -> Self {
        BookList { book_list }
    }
}

#[derive(Debug, Clone)]
pub struct BookListItem {
    pub author: String,
    pub book_url: String,
    pub cover_url: Option<String>,
    pub intro: String,
    pub kind: String,
    pub name: String,
    pub word_count: String,
}

impl BookListItem {
    pub async fn get_book_info(&self, client: &HttpClient) -> Result<Response> {
        client.get(&self.book_url).await
    }
}
