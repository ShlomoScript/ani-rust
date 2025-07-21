use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub _id: String,
    pub name: String,
    #[serde(rename = "availableEpisodes")]
    pub available_episodes: Option<EpisodeCounts>,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeCounts {
    pub sub: Option<u32>,
    pub dub: Option<u32>,
    pub raw: Option<u32>,
}
