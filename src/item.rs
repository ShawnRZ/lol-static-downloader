// {
//     "id": 1001,
//     "name": "鞋子",
//     "description": "<mainText><stats><attention>25</attention>移动速度</stats></mainText><br>",
//     "active": false,
//     "inStore": true,
//     "from": [],
//     "to": [
//         3111,
//         3006,
//         3009,
//         3020,
//         3047,
//         3117,
//         3158
//     ],
//     "categories": [
//         "Boots"
//     ],
//     "maxStacks": 1,
//     "requiredChampion": "",
//     "requiredAlly": "",
//     "requiredBuffCurrencyName": "",
//     "requiredBuffCurrencyCost": 0,
//     "specialRecipe": 0,
//     "isEnchantment": false,
//     "price": 300,
//     "priceTotal": 300,
//     "iconPath": "/lol-game-data/assets/ASSETS/Items/Icons2D/1001_Class_T1_BootsofSpeed.png"
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "price")]
    pub price: Option<i32>,
    #[serde(rename = "priceTotal")]
    pub price_total: Option<i32>,
    #[serde(rename = "iconPath")]
    pub path: Option<String>,
}
