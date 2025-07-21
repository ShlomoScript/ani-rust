use reqwest::Client;
use reqwest::header::{REFERER, USER_AGENT};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Anime {
    #[serde(rename = "_id")]
    pub id: String,
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

#[derive(Debug, Deserialize)]
pub struct EpisodeList {
    pub sub: Vec<String>,
    pub dub: Vec<String>,
    pub raw: Vec<String>,
}
#[derive(Debug, Deserialize)]
struct EpisodesApiResponse {
    data: EpisodesData,
}
#[derive(Debug, Deserialize)]
struct EpisodesData {
    show: EpisodeInfo,
}
#[derive(Debug, Deserialize)]
struct EpisodeInfo {
    _id: String,
    #[serde(rename = "availableEpisodesDetail")]
    available_episodes_detail: EpisodeList,
}

impl Anime {
    pub async fn get_episode_list(&self) -> Result<EpisodeList, Box<dyn std::error::Error>> {
        let client = Client::new();

        let url = "https://api.allanime.day/api";

        let variables = serde_json::json!({
            "showId": self.id
        });
        let gql_query = r#"
            query(
                $showId: String!
            ) {
                show(
                    _id: $showId
                ) {
_id availableEpisodesDetail 
                }
            }"#;
        let response = client
            .get(url)
            .query(&[
                ("variables", variables.to_string()),
                ("query", gql_query.to_string()),
            ])
            .header(
                USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0",
            )
            .header(REFERER, "https://allmanga.to")
            .send()
            .await?;
        let json: EpisodesApiResponse = response.json().await?;

        Ok(json.data.show.available_episodes_detail)
    }
}
