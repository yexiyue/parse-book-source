use crate::Result;
use crate::{utils::DataWithRegex, ParseError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuleContent {
    pub content: String,
}

impl TryFrom<&RuleContent> for RuleContentWithRegex {
    type Error = ParseError;

    fn try_from(value: &RuleContent) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            content: value.content.as_str().try_into()?,
        })
    }
}

impl TryFrom<RuleContent> for RuleContentWithRegex {
    type Error = ParseError;
    fn try_from(value: RuleContent) -> std::result::Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

pub struct RuleContentWithRegex {
    pub content: DataWithRegex,
}

impl RuleContentWithRegex {
    pub fn parse_content(&self, data: &Value) -> Result<String> {
        self.content.parse_data(data)
    }
}