pub fn query(input: String) -> String {
    let url = format!("https://www.musixmatch.com/search/{}", input);

    let response = reqwest::blocking::get(url)
        .expect("Failed to search: Initial request failed")
        .text()
        .expect("Failed to search: Request for text timed out");

    let document = scraper::Html::parse_document(&response);

    let song_selector =
        scraper::Selector::parse(".title").expect("Failed to search: Selector failed");

    let song = document.select(&song_selector).next();

    if song.is_none() {
        eprintln!("No songs found for query: {}", input);
        std::process::exit(1);
    }

    let song = song.unwrap();

    let song_url = song
        .value()
        .attr("href")
        .expect("Failed to search: No href attribute found");

    let song_url = format!("https://www.musixmatch.com{}", song_url);

    song_url
}