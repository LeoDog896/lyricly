fn fetch_lyrics(url: String) -> String {
    let response = reqwest::blocking::get(url)
    .expect("Failed to fetch lyrics: Initial request failed")
    .text()
    .expect("Failed to fetch lyrics: Request for text timed out");

    let document = scraper::Html::parse_document(&response);

    let lyric_selector = scraper::Selector::parse(".lyrics__content__ok").unwrap();

    let lyrics = document.select(&lyric_selector).map(|x| x.inner_html());

    let text_lyrics = lyrics.collect::<Vec<_>>().join("\n").trim().to_string();

    text_lyrics + "\n"
}

fn main() {
    let text_lyrics = fetch_lyrics("https://www.musixmatch.com/lyrics/Shawn-Wasabi-feat-Hollis/Otter-Pop".to_string());

    println!("{}", text_lyrics + "\n");

}