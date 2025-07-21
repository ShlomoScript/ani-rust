use super::anime::Anime;
use reqwest::Client;
use reqwest::header::{REFERER, USER_AGENT};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Shows {
    edges: Vec<Anime>,
}

#[derive(Debug, Deserialize)]
struct SearchData {
    shows: Shows,
}

#[derive(Debug, Deserialize)]
struct SearchApiResponse {
    data: SearchData,
}

pub async fn search_anime(query: &str) -> Result<Vec<Anime>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = "https://api.allanime.day/api";

    let variables = serde_json::json!({
        "search": {
            "allowAdult": false,
            "allowUnknown": false,
            "query": query
        },
        "limit": 40,
        "page": 1,
        "translationType": "sub", // or "dub"
        "countryOrigin": "ALL"
    });

    let gql_query = r#"
        query(
            $search: SearchInput
            $limit: Int
            $page: Int
            $translationType: VaildTranslationTypeEnumType
            $countryOrigin: VaildCountryOriginEnumType
        ) {
            shows(
                search: $search
                limit: $limit
                page: $page
                translationType: $translationType
                countryOrigin: $countryOrigin
            ) {
                edges {
                    _id
                    name
                    availableEpisodes
                    __typename
                }
            }
        }
    "#;

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

    let json: SearchApiResponse = response.json().await?;

    Ok(json.data.shows.edges)
}
