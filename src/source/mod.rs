pub mod rule;
use crate::{
    utils::Params, BookInfo, BookList, BookListItem, Chapter, ChapterList, ExploreItem, Result,
    Variables,
};
use anyhow::anyhow;
pub use rule::*;
use serde::{Deserialize, Serialize};

use crate::{Explores, HttpClient, ParseError, Search};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookSource {
    pub book_source_group: String,
    pub book_source_name: String,
    pub book_source_type: u64,
    pub book_source_url: String,
    pub enabled_explore: bool,
    pub explore_url: Option<String>,
    pub header: String,
    pub last_update_time: u64,
    pub respond_time: u64,
    pub rule_book_info: RuleBookInfo,
    pub rule_content: RuleContent,
    pub rule_explore: Option<RuleExplore>,
    pub rule_search: RuleSearch,
    pub rule_toc: RuleToc,
    pub search_url: String,
}

#[derive(Debug, Clone)]
pub struct JsonSource {
    pub variables: Variables,
    pub source_name: String,
    pub source_type: u64,
    pub source_group: String,
    pub client: HttpClient,
    pub search: Search,
    pub explores: Option<Explores>,
    pub rule: JsonRule,
}

impl TryFrom<BookSource> for JsonSource {
    type Error = ParseError;
    fn try_from(value: BookSource) -> std::result::Result<Self, Self::Error> {
        let explores = if value.enabled_explore {
            Some(Explores::try_from(
                value
                    .explore_url
                    .ok_or(anyhow!("explore_url is not found"))?
                    .as_str(),
            )?)
        } else {
            None
        };

        let explore_rule = if value.enabled_explore {
            Some(JsonRuleExplore::try_from(
                value
                    .rule_explore
                    .ok_or(anyhow!("ruleExplore is not found"))?,
            )?)
        } else {
            None
        };

        Ok(Self {
            variables: Variables::new()?,
            source_name: value.book_source_name,
            source_type: value.book_source_type,
            source_group: value.book_source_group,
            client: HttpClient::new(&value.book_source_url, &value.header, value.respond_time)?,
            search: Search::from(&value.search_url),
            explores,
            rule: JsonRule {
                book_info: JsonRuleBookInfo::try_from(value.rule_book_info)?,
                content: JsonRuleContent::try_from(value.rule_content)?,
                explore: explore_rule,
                search: JsonRuleSearch::try_from(value.rule_search)?,
                toc: JsonRuleToc::try_from(value.rule_toc)?,
            },
        })
    }
}

impl JsonSource {
    pub async fn search_books(&mut self, params: Params) -> Result<BookList> {
        let res = self
            .search
            .get_book_list(&self.client, params)
            .await?
            .json()
            .await?;

        self.rule.search.parse_book_list(&res, &mut self.variables)
    }

    pub async fn explore_books(
        &mut self,
        explore: &ExploreItem,
        params: Params,
    ) -> Result<BookList> {
        let res = explore
            .get_book_list(&self.client, params)
            .await?
            .json()
            .await?;

        self.rule
            .explore
            .as_ref()
            .unwrap()
            .parse_book_list(&res, &mut self.variables)
    }

    pub async fn book_info(&mut self, book_list_item: &BookListItem) -> Result<BookInfo> {
        let res = book_list_item
            .get_book_info(&self.client)
            .await?
            .json()
            .await?;

        self.rule
            .book_info
            .parse_book_info(&res, &mut self.variables)
    }

    pub async fn chapter_list(&mut self, book_info: &BookInfo) -> Result<ChapterList> {
        let res = book_info
            .get_chapter_list(&self.client)
            .await?
            .json()
            .await?;

        self.rule.toc.parse_chapter_list(&res, &mut self.variables)
    }

    pub async fn chapter_content(&mut self, chapter: &Chapter) -> Result<String> {
        let res = chapter.get_content(&self.client).await?.json().await?;

        self.rule
            .content
            .parse_content(&res, &mut self.variables, &self.client)
            .await
    }
}
