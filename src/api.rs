use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Clone, Deserialize)]
pub struct SearchResults {
    pub results: Vec<ResultItem>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct ResultItem {
    pub robot: Robot,
    pub prices: Vec<Price>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Price {
    pub currency: usize,
    pub amount: usize,
}

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, PartialEq)]
pub struct Robot {
    pub id: String,
    pub name: String,
    pub creatorId: String,
    pub creatorName: String,
    pub image: String,
    pub baseCpu: usize,
    pub weaponCpu: usize,
    pub cosmeticCpu: usize,
    pub clusterCount: usize,
    pub blockCounts: std::collections::HashMap<usize, usize>,
    pub materialsUsed: Vec<usize>,
    // TODO(MAYBE) min/max offsets
}

#[derive(Serialize, Clone)]
pub struct SearchRequest {
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "baseCpuMinimum")]
    pub base_minimum_cpu: Option<isize>,
    #[serde(rename = "baseCpuMaximum")]
    pub base_maximum_cpu: Option<isize>,
    #[serde(rename = "weaponCpuMinimum")]
    pub weapon_minimum_cpu: Option<isize>,
    #[serde(rename = "weaponCpuMaximum")]
    pub weapon_maximum_cpu: Option<isize>,
    #[serde(rename = "cosmeticCpuMinimum")]
    pub cosmetic_minimum_cpu: Option<isize>,
    #[serde(rename = "cosmeticCpuMaximum")]
    pub cosmetic_maximum_cpu: Option<isize>,
    #[serde(rename = "clusterMinimum")]
    pub cluster_minimum: Option<isize>,
    #[serde(rename = "clusterMaximum")]
    pub cluster_maximum: Option<isize>,
    #[serde(rename = "dateMinimum")]
    pub date_minimum: Option<String>,
    #[serde(rename = "dateMaximum")]
    pub date_maximum: Option<String>,
    #[serde(rename = "creatorId")]
    pub creator_id: Option<String>, // GUID
    #[serde(rename = "page")]
    pub page: Option<isize>,
    #[serde(rename = "count")]
    pub count: Option<isize>,
    #[serde(rename = "sortBy")]
    pub sort_by: String,
    #[serde(rename = "orderBy")]
    pub order_by: String,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            text: None,
            base_minimum_cpu: None,
            base_maximum_cpu: None,
            weapon_minimum_cpu: None,
            weapon_maximum_cpu: None,
            cosmetic_minimum_cpu: None,
            cosmetic_maximum_cpu: None,
            cluster_minimum: None,
            cluster_maximum: None,
            date_minimum: None,
            date_maximum: None,
            creator_id: None,
            page: None,
            count: None,
            sort_by: "default".to_owned(),
            order_by: "ascending".to_owned(),
        }
    }
}

pub async fn search_query(query: &SearchRequest) -> Result<SearchResults, String> {
    let response = Request::post("/crf-api/search")
        .json(query).map_err(|e| e.to_string())?
        .send()
        .await.map_err(|e| e.to_string())?;
    Ok(response.json()
        .await.map_err(|e| e.to_string())?)
}
