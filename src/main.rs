use clap::Parser;

/// Get lyrics from various songs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The query of the song to get lyrics for.
    song_query: Vec<String>,
}

fn fetch_lyrics(url: String) -> String {
    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch lyrics: Initial request failed")
        .text()
        .expect("Failed to fetch lyrics: Request for text timed out");

    let document = scraper::Html::parse_document(&response);

    let lyric_selector = scraper::Selector::parse(".lyrics__content__ok").expect("Failed to fetch lyrics: Selector failed");

    let lyrics = document.select(&lyric_selector).map(|x| x.inner_html());

    let text_lyrics = lyrics.collect::<Vec<_>>().join("\n").trim().to_string();

    text_lyrics + "\n"
}

fn search(query: String) -> String {
    let url = format!("https://www.musixmatch.com/search/{}", query);

    let response = reqwest::blocking::get(url)
        .expect("Failed to search: Initial request failed")
        .text()
        .expect("Failed to search: Request for text timed out");

    let document = scraper::Html::parse_document(&response);

    let song_selector = scraper::Selector::parse(".title").expect("Failed to search: Selector failed");

    let song = document.select(&song_selector).next();

    if song.is_none() {
        println!("No songs found for query: {}", query);
        std::process::exit(1);
    }

    let song = song.unwrap();

    let song_url = song.value().attr("href").expect("Failed to search: No href attribute found");
    
    let song_url = format!("https://www.musixmatch.com{}", song_url);

    song_url
}

fn main() {
    let args = Args::parse();

    let song_url = search(args.song_query.join(" "));

    let text_lyrics = fetch_lyrics(
        song_url,
    );

    println!("{}", text_lyrics + "\n");
}
