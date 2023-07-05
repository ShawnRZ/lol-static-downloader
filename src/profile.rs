// {
//     "id": 941,
//     "iconPath": "/lol-game-data/assets/v1/profile-icons/941.jpg"
// },

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "iconPath")]
    pub path: Option<String>,
}
