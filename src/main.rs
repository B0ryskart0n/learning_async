use trpl::Html;

/// Equivalent to `fn page_title(url: &str) -> impl Future<Output = Option<String>> { async move { BODY }}`
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];

    trpl::block_on(async {
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
