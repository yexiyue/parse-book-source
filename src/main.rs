use std::{fs::File, thread::sleep, time::Duration};

use parse_book_source::{
    http_client::{self, HttpClient}, utils::{value_to_string, DataWithRegex}, BookSource, RuleBookInfoWithRegex, RuleContentWithRegex, RuleExploreWithRegex, RuleSearchWithRegex, RuleTocWithRegex, Search
};
use regex::Regex;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = File::open("test2.json")?;
    let book_source: BookSource = serde_json::from_reader(file)?;
    let http_client = HttpClient::new(
        &book_source.book_source_url,
        &book_source.header,
        book_source.respond_time as u64,
    )?;
    let search = Search::from(&book_source.search_url);

    let books = search.get_book_list(&http_client, 1, "吞噬星空").await?;

    let res: RuleSearchWithRegex = book_source.rule_search.try_into()?;
    let res = res.parse_book_list(&books.json().await?)?;

    let book_info: RuleBookInfoWithRegex = book_source.rule_book_info.try_into()?;
    println!("{:#?}", res);
    println!("等待中");
    sleep(Duration::from_secs(2));
    println!("等待结束");
    let book = &res[0].get_book_info(&http_client).await?.json().await?;
    let res2 = book_info.parse_book_info(book).await?;

    println!("{:#?}", res2);
    println!("{:#?}", book);
    println!("等待中");
    sleep(Duration::from_secs(2));
    println!("等待结束");
    let chapter_list: RuleTocWithRegex = book_source.rule_toc.try_into()?;
    let res3 = chapter_list.parse_chapter_list(
        &res2.get_chapter_list(&http_client).await?.json().await?,
        // &value_to_string(book, "$.result.book_id")?,
        "abc",
    )?;
    println!("{:#?}", res3);
    println!("等待中");
    sleep(Duration::from_secs(2));
    println!("等待结束");
    let chapter: &Value = &res3[0].get_content(&http_client).await?.json().await?;

    let content: RuleContentWithRegex = book_source.rule_content.try_into()?;
    let res4 = content.parse_content(chapter)?;
    println!("{:#?}", res4);
    Ok(())
}
