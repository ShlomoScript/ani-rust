use ani_rust::anime;

#[tokio::main]
async fn main() {
    let show = anime::Anime {
        id: String::from("osGHAaTHeoTZLTs4o"),
        name: String::from("Chainsaw Man"),
        available_episodes: None,
    };
    let episodes = show.get_episode_list().await.unwrap();
    println!("{episodes:#?}");
}
