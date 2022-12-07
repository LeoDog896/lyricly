use clap::Parser;
use thiserror::Error;

/// Get lyrics from various songs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The query of the song to get lyrics for.
    song_query: Vec<String>,
}

fn strip_trailing_nl(input: &mut String) {
    let new_len = input
        .char_indices()
        .rev()
        .find(|(_, c)| !matches!(c, '\n' | '\r'))
        .map_or(0, |(i, _)| i + 1);
    if new_len != input.len() {
        input.truncate(new_len);
    }
}

#[derive(Error, Debug)]
pub enum LyricFetchError {
    #[error("initial request failed")]
    InitialRequest,
    #[error("request timed out")]
    TimedOut,
    #[error("selector failed to parse: {0}")]
    SelectorFailed(String),
    #[error("lyrics were restricted")]
    Restricted,
    #[error("no lyrics were available")]
    NoLyrics
}

fn fetch_lyrics(url: &str) -> Result<String, LyricFetchError> {
    let response = reqwest::blocking::get(url)
        .map_err(|_| LyricFetchError::InitialRequest)?
        .text()
        .map_err(|_| LyricFetchError::TimedOut)?;

    let document = scraper::Html::parse_document(&response);

    // checking for restricted lyrics
    {
        let restricted_selector = scraper::Selector::parse(".mxm-lyrics-not-available")
            .map_err(|_| LyricFetchError::SelectorFailed(".mxm-lyrics-not-available".to_string()))?;

        let mut restricted_text = document.select(&restricted_selector);

        if restricted_text.next().is_some() {
            return Err(LyricFetchError::Restricted)
        }
    };
    let lyric_selector = scraper::Selector::parse(".lyrics__content__ok")
        .map_err(|_| LyricFetchError::SelectorFailed(".lyrics__content__ok".to_string()))?;

    let lyrics = document.select(&lyric_selector);

    let lyrics = lyrics.map(|x| x.inner_html());

    let lyrics_list = &mut lyrics.collect::<Vec<_>>().join("\n");

    strip_trailing_nl(lyrics_list);

    if lyrics_list.is_empty() {
        return Err(LyricFetchError::NoLyrics);
    }

    Ok(lyrics_list.to_owned() + "\n")
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

fn main() {
    let args = Args::parse();

    let song_url = search(args.song_query.join(" "));

    let text_lyrics = fetch_lyrics(&song_url);

    if text_lyrics.is_err() {
        eprintln!("{:?}", text_lyrics);
        std::process::exit(1);
    }

    let text_lyrics = text_lyrics.unwrap();

    println!("{}", text_lyrics + "\n");
}
