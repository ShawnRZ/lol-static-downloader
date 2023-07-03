use serde::{Deserialize, Serialize};

// {
//     "id": 8369,
//     "name": "先攻",
//     "majorChangePatchVersion": "11.23",
//     "tooltip": "在进入与英雄战斗的@GraceWindow.2@秒内，对一名敌方英雄进行的攻击或技能将提供@GoldProcBonus@金币和<b>先攻</b>效果，持续@Duration@秒，来使你对英雄们造成<truedamage>@DamageAmp*100@%</truedamage>额外<truedamage>伤害</truedamage>，并提供<gold>{{ Item_Melee_Ranged_Split }}</gold>该额外伤害值的<gold>金币</gold>。<br><br>冷却时间：<scaleLevel>@Cooldown@</scaleLevel>秒<br><hr><br>已造成的伤害：@f1@<br>已提供的金币：@f2@",
//     "shortDesc": "在你率先发起与英雄的战斗时，造成9%额外伤害，持续3秒，并基于该额外伤害提供金币。",
//     "longDesc": "在进入与英雄战斗的0.25秒内，对一名敌方英雄进行的攻击或技能将提供5金币和<b>先攻</b>效果，持续3秒，来使你对英雄们造成<truedamage>9%</truedamage>额外<truedamage>伤害</truedamage>，并提供<gold>100% (远程英雄为70%)</gold>该额外伤害值的<gold>金币</gold>。<br><br>冷却时间：<scaleLevel>25 ~ 15</scaleLevel>秒",
//     "recommendationDescriptor": "真实伤害，金币收入",
//     "iconPath": "/lol-game-data/assets/v1/perk-images/Styles/Inspiration/FirstStrike/FirstStrike.png",
//     "endOfGameStatDescs": [
//         "已造成的伤害：@eogvar1@",
//         "已提供的金币：@eogvar2@"
//     ],
//     "recommendationDescriptorAttributes": {}
// },

#[derive(Debug, Serialize, Deserialize)]
pub struct Rune {
    #[serde(rename = "id")]
    pub id: Option<i64>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    #[serde(rename = "endOfGameStatDescs")]
    pub end_of_game_stat_descs: Option<Vec<String>>,
}
