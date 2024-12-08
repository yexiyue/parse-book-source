pub mod rule;
pub use rule::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookSource {
    pub book_source_group: String,
    pub book_source_name: String,
    pub book_source_type: i64,
    pub book_source_url: String,
    pub enabled_explore: bool,
    pub explore_url: Option<String>,
    pub header: String,
    pub last_update_time: i64,
    pub respond_time: i64,
    pub rule_book_info: RuleBookInfo,
    pub rule_content: RuleContent,
    pub rule_explore: Option<RuleExplore>,
    pub rule_search: RuleSearch,
    pub rule_toc: RuleToc,
    pub search_url: String,
}
