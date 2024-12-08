use crate::{utils::DataWithRegex, BookInfo, ParseError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuleBookInfo {
    pub author: String,
    pub cover_url: String,
    pub intro: String,
    pub kind: String,
    pub last_chapter: String,
    pub name: String,
    pub toc_url: String,
    pub word_count: String,
}

#[derive(Debug, Clone)]
pub struct RuleBookInfoWithRegex {
    pub author: DataWithRegex,
    pub cover_url: DataWithRegex,
    pub intro: DataWithRegex,
    pub kind: DataWithRegex,
    pub last_chapter: DataWithRegex,
    pub name: DataWithRegex,
    pub toc_url: DataWithRegex,
    pub word_count: DataWithRegex,
}

impl TryFrom<&RuleBookInfo> for RuleBookInfoWithRegex {
    type Error = ParseError;
    fn try_from(value: &RuleBookInfo) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            author: value.author.as_str().try_into()?,
            intro: value.intro.as_str().try_into()?,
            kind: value.kind.as_str().try_into()?,
            name: value.name.as_str().try_into()?,
            word_count: value.word_count.as_str().try_into()?,
            cover_url: value.cover_url.as_str().try_into()?,
            last_chapter: value.last_chapter.as_str().try_into()?,
            toc_url: value.toc_url.as_str().try_into()?,
        })
    }
}

impl TryFrom<RuleBookInfo> for RuleBookInfoWithRegex {
    type Error = ParseError;
    fn try_from(value: RuleBookInfo) -> std::result::Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl RuleBookInfoWithRegex {
    pub async fn parse_book_info(&self, data: &Value) -> Result<BookInfo> {
        Ok(BookInfo {
            author: self.author.parse_data(&data)?,
            cover_url: self.cover_url.parse_data(&data).ok(),
            intro: self.intro.parse_data(&data)?,
            kind: self.kind.parse_data(&data)?,
            last_chapter: self.last_chapter.parse_data(&data)?,
            name: self.name.parse_data(&data)?,
            toc_url: self.toc_url.parse_data(&data)?,
            word_count: self.word_count.parse_data(&data)?,
        })
    }
}
