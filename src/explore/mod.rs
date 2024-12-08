pub mod explore_item;
use crate::{ParseError, Result};
pub use explore_item::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Explores {
    #[serde(flatten)]
    pub explore_list: Vec<ExploreItem>,
}

impl Explores {
    pub fn from_json(json: &str) -> Result<Self> {
        Self::try_from(json)
    }
}

impl TryFrom<&str> for Explores {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self> {
        Self::from_json(value)
    }
}
