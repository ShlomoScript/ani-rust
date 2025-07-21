use ani_rust::search::search_anime;
use std::io::{Write, stdin, stdout};

#[tokio::main]
async fn main() {
    print!("Anime: ");
    let _ = stdout().flush();
    for line in stdin().lines() {
        match line {
            Ok(line) => match search_anime(&line).await {
                Ok(anime_list) => {
                    for anime in anime_list {
                        let ep = anime.available_episodes.as_ref();
                        println!(
                            "\t{} - sub: {}, dub: {} (ID: {})",
                            anime.name,
                            ep.and_then(|e| e.sub).unwrap_or(0),
                            ep.and_then(|e| e.dub).unwrap_or(0),
                            anime.id
                        );
                    }
                }
                Err(e) => eprintln!("Error: {e}"),
            },
            Err(e) => eprintln!("Error: {e}"),
        }
        print!("Anime: ");
        let _ = stdout().flush();
    }
}
