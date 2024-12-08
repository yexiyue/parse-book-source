use crate::{ParseError, Result};
use anyhow::anyhow;
use regex::Regex;
use serde_json::Value;

use super::{parse_template, split_preserving_delimiters};

#[derive(Debug, Clone, Default)]
pub struct DataWithRegex {
    pub data: String,
    pub regex: Option<Regex>,
    pub replace_content: Option<String>,
}

impl DataWithRegex {
    fn replace_all(&self, haystack: &str) -> Result<String> {
        if let Some(re) = &self.regex {
            Ok(re
                .replace_all(
                    haystack,
                    self.replace_content.as_ref().unwrap_or(&"".into()),
                )
                .into())
        } else {
            Ok(haystack.into())
        }
    }

    pub fn set_book_id(&self, book_id: &str) -> Result<Self> {
        let mut new = self.clone();
        let regex = Regex::new(r"@get:\{book\}")?;
        new.data = regex.replace_all(&self.data, book_id).into();
        Ok(new.clone())
    }

    pub fn parse_data(&self, data: &Value) -> Result<String> {
        let value = parse_template(&self.data, data)?;
        self.replace_all(&value)
    }
}

impl TryFrom<&str> for DataWithRegex {
    type Error = ParseError;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let str = split_preserving_delimiters(value);

        let mut res = DataWithRegex::default();

        res.data = str.get(0).ok_or(anyhow!("data is not found"))?.to_string();

        if let Some(regex) = str.get(1) {
            res.regex = Some(Regex::new(regex)?);
        }

        if let Some(content) = str.get(2) {
            res.replace_content = Some(content.to_string());
        }

        Ok(res)
    }
}

impl TryFrom<String> for DataWithRegex {
    type Error = ParseError;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        DataWithRegex::try_from(value.as_str())
    }
}
