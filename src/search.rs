use thiserror::Error;

#[derive(Error, Debug)]
pub enum LyricSearchError {
    #[error("Initial request failed.")]
    InitialRequest,
    #[error("Request timed out.")]
    TimedOut,
    #[error("Selector '{0}' failed to parse.")]
    SelectorFailed(String),
    #[error("No songs were found.")]
    NoSongs,
}

pub fn query(input: String) -> Result<String, LyricSearchError> {
    let url = format!("https://www.musixmatch.com/search/{}", input);

    let response = reqwest::blocking::get(url)
        .map_err(|_| LyricSearchError::InitialRequest)?
        .text()
        .map_err(|_| LyricSearchError::TimedOut)?;

    let document = scraper::Html::parse_document(&response);

    let song_selector =
        scraper::Selector::parse(".title").map_err(|_| LyricSearchError::SelectorFailed("title".to_string()))?;

    let song = document.select(&song_selector).next();

    if song.is_none() {
        return Err(LyricSearchError::NoSongs);
    }

    let song = song.unwrap();

    let song_url = song
        .value()
        .attr("href")
        .expect("Failed to search: No href attribute found");

    let song_url = format!("https://www.musixmatch.com{}", song_url);

    Ok(song_url)
}