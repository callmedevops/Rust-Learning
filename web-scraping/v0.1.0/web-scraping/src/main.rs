use reqwest::Error;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://news.ycombinator.com/";
    let response = reqwest::get(url).await?;
    let html = response.text().await?;

    let document = Document::from(html.as_str());

    for title in document.find(Class("titleline").child(Name("a"))) {
        println!("{}", title.text());
    }

    Ok(())
}
