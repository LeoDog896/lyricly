use thiserror::Error;

#[derive(Error, Debug)]
pub enum LyricFetchError {
    #[error("Initial request failed.")]
    InitialRequest,
    #[error("Request timed out.")]
    TimedOut,
    #[error("Selector '{0}' failed to parse.")]
    SelectorFailed(String),
    #[error("Lyrics are not available for public use.")]
    Restricted,
    #[error("Lyrics are not available.")]
    NoLyrics,
}

pub fn fetch(url: &str) -> Result<String, LyricFetchError> {
    let response = reqwest::blocking::get(url)
        .map_err(|_| LyricFetchError::InitialRequest)?
        .text()
        .map_err(|_| LyricFetchError::TimedOut)?;

    let document = scraper::Html::parse_document(&response);

    // checking for restricted lyrics
    {
        let restricted_selector =
            scraper::Selector::parse(".mxm-lyrics-not-available").map_err(|_| {
                LyricFetchError::SelectorFailed(".mxm-lyrics-not-available".to_string())
            })?;

        let mut restricted_text = document.select(&restricted_selector);

        if restricted_text.next().is_some() {
            return Err(LyricFetchError::Restricted);
        }
    };
    let lyric_selector = scraper::Selector::parse(".lyrics__content__ok")
        .map_err(|_| LyricFetchError::SelectorFailed(".lyrics__content__ok".to_string()))?;

    let lyrics = document.select(&lyric_selector);

    let lyrics = lyrics.map(|x| x.inner_html());

    let lyrics_list = &mut lyrics.collect::<Vec<_>>().join("\n");

    let lyrics_list = lyrics_list.trim();

    if lyrics_list.is_empty() {
        return Err(LyricFetchError::NoLyrics);
    }

    Ok(lyrics_list.to_owned() + "\n")
}
