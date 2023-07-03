use serde::{Deserialize, Serialize};

// {
//     "id": -1,
//     "name": "无",
//     "alias": "None",
//     "squarePortraitPath": "/lol-game-data/assets/v1/champion-icons/-1.png",
//     "roles": []
// },

#[derive(Debug, Serialize, Deserialize)]
pub struct Champion {
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "squarePortraitPath")]
    pub path: Option<String>,
    #[serde(rename = "roles")]
    pub roles: Option<Vec<String>>,
}
