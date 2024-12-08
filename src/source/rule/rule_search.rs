use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    utils::{json_path, DataWithRegex},
    BookList, BookListItem, ParseError, Result,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSearch {
    pub author: String,
    pub book_list: String,
    pub book_url: String,
    pub cover_url: String,
    pub intro: String,
    pub kind: String,
    pub name: String,
    pub word_count: String,
}

#[derive(Debug, Clone)]
pub struct RuleSearchWithRegex {
    pub book_list: String,
    pub author: DataWithRegex,
    pub intro: DataWithRegex,
    pub kind: DataWithRegex,
    pub name: DataWithRegex,
    pub word_count: DataWithRegex,
    pub book_url: DataWithRegex,
    pub cover_url: DataWithRegex,
}

impl TryFrom<&RuleSearch> for RuleSearchWithRegex {
    type Error = ParseError;
    fn try_from(value: &RuleSearch) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            book_list: value.book_list.clone(),
            author: value.author.as_str().try_into()?,
            intro: value.intro.as_str().try_into()?,
            kind: value.kind.as_str().try_into()?,
            name: value.name.as_str().try_into()?,
            word_count: value.word_count.as_str().try_into()?,
            book_url: value.book_url.as_str().try_into()?,
            cover_url: value.cover_url.as_str().try_into()?,
        })
    }
}

impl TryFrom<RuleSearch> for RuleSearchWithRegex {
    type Error = ParseError;
    fn try_from(value: RuleSearch) -> std::result::Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl RuleSearchWithRegex {
    pub fn parse_book_list(&self, data: &Value) -> Result<BookList> {
        let book_list = if self.book_list.as_str().ends_with("[*]") {
            json_path(data, self.book_list.as_str())?
        } else {
            json_path(data, &format!("{}[*]", self.book_list))?
        };

        let mut res = vec![];
        for item in book_list
            .as_array()
            .ok_or(anyhow!("book_list is not array"))?
        {
            res.push(BookListItem {
                author: self.author.parse_data(item)?,
                intro: self.intro.parse_data(item)?,
                kind: self.kind.parse_data(item)?,
                name: self.name.parse_data(item)?,
                word_count: self.word_count.parse_data(item)?,
                book_url: self.book_url.parse_data(item)?,
                cover_url: self.cover_url.parse_data(item).ok(),
            });
        }

        Ok(res.into())
    }
}
