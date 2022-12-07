use clap::Parser;

mod lyrics;
/// Get lyrics from various songs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The query of the song to get lyrics for.
    song_query: Vec<String>,
}

fn search(query: String) -> String {
    let url = format!("https://www.musixmatch.com/search/{}", query);

    let response = reqwest::blocking::get(url)
        .expect("Failed to search: Initial request failed")
        .text()
        .expect("Failed to search: Request for text timed out");

    let document = scraper::Html::parse_document(&response);

    let song_selector =
        scraper::Selector::parse(".title").expect("Failed to search: Selector failed");

    let song = document.select(&song_selector).next();

    if song.is_none() {
        eprintln!("No songs found for query: {}", query);
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let song_url = search(args.song_query.join(" "));

    let text_lyrics = lyrics::fetch(&song_url)?;

    println!("{}", text_lyrics + "\n");

    Ok(())
}
