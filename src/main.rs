use clap::Parser;

mod lyrics;
mod search;
/// Get lyrics from various songs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The query of the song to get lyrics for.
    song_query: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let song_url = search::query(args.song_query.join(" "));

    let text_lyrics = lyrics::fetch(&song_url)?;

    println!("{}", text_lyrics + "\n");

    Ok(())
}
