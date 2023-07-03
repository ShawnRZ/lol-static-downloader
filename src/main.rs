pub mod champion;
pub mod item;
pub mod rune;

use champion::Champion;
use env_logger::Env;
use item::Item;
use log::{debug, error};
use reqwest::get;
use rune::Rune;
use tokio::{fs, io::AsyncWriteExt, task::JoinSet};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn champion() -> Result<(), Error> {
    let res = get("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/champion-summary.json").await?.error_for_status()?;
    let champion_list: Vec<Champion> = serde_json::from_str(&res.text().await?)?;
    let len = champion_list.len();

    println!("{:#?}", champion_list);

    // https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default /v1/champion-icons/-1.png
    //                                                              /lol-game-data/assets /v1/champion-icons/-1.png

    let base_url =
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";

    let mut set = JoinSet::new();

    for champion in champion_list {
        let mut url = base_url.to_string();
        let path = champion
            .path
            .ok_or("字段缺失")?
            .clone()
            .strip_prefix("/lol-game-data/assets")
            .ok_or("字段值异常")?
            .to_lowercase();
        url.push_str(&path);
        set.spawn(async move { (get(url).await, champion.id) });
    }

    let mut count = 0;
    while let Some(res) = set.join_next().await {
        let (res, id) = res?;
        let res = res?;
        if !res.status().is_success() {
            continue;
        }
        println!("{:?}", res.url().path());
        count += 1;
        let path = std::format!("./static/champion/{}.png", id.unwrap());
        let mut f = fs::File::create(path).await?;
        f.write_all_buf(&mut res.bytes().await?).await?;
    }
    println!("{}/{}", count, len);
    Ok(())
}

pub async fn rune() -> Result<(), Error> {
    let res =  get("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/perks.json").await?.error_for_status()?;

    // println!("{}", res.text().await?);

    let rune_list: Vec<Rune> = serde_json::from_str(&res.text().await?)?;
    let len = rune_list.len();

    println!("{:#?}", &rune_list);

    // https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default /v1/perk-images/styles/inspiration/firststrike/firststrike.png
    //                                                              /lol-game-data/assets /v1/perk-images/Styles/Inspiration/FirstStrike/FirstStrike.png
    let base_url =
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";

    // let mut handles = Vec::new();
    let mut set = JoinSet::new();

    for rune in rune_list {
        let mut url = base_url.to_string();
        let path = rune
            .icon_path
            .clone()
            .ok_or("字段缺失")?
            .strip_prefix("/lol-game-data/assets")
            .ok_or("字段值异常")?
            .to_lowercase();
        url.push_str(&path);

        set.spawn(async move { (get(url).await, rune.id) });
        // handles.push(tokio::spawn(async {
        //     let res = get(url).await;
        // }));
    }
    let mut count = 0;
    while let Some(res) = set.join_next().await {
        let (res, id) = res?;
        let res = res?;
        if res.status().is_success() {
            println!("{:?}", res.url().path());
            count += 1;
            let path = std::format!("./static/rune/{}.png", id.unwrap());
            let mut f = fs::File::create(path).await?;
            f.write_all_buf(&mut res.bytes().await?).await?;
        }
    }
    println!("{}/{}", count, len);
    Ok(())
}

pub async fn item() -> Result<(), Error> {
    let res = get("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/items.json").await?.error_for_status()?;

    let item_list: Vec<Item> = serde_json::from_str(&res.text().await?)?;
    let len = item_list.len();

    // println!("{:?}", item_list);

    let base_url =
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";

    let mut set = JoinSet::new();

    // https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default /assets/items/icons2d/
    //                                                              /lol-game-data/assets /ASSETS/Items/Icons2D/1001_Class_T1_BootsofSpeed.png

    for item in item_list {
        let mut url = base_url.to_string();
        let path = item
            .path
            .ok_or("字段缺失")?
            .clone()
            .strip_prefix("/lol-game-data/assets")
            .ok_or("字段异常")?
            .to_lowercase();
        url.push_str(&path);
        set.spawn(async move { (get(url).await, item.id) });
    }

    let mut count = 0;

    while let Some(res) = set.join_next().await {
        let (res, id) = res?;
        let res = res?;
        if !res.status().is_success() {
            println!("faild {}", res.url().path());
            continue;
        }
        println!("{:?}", res.url().path());
        count += 1;
        let path = std::format!("./static/item/{}.png", id.unwrap());
        let mut f = fs::File::create(path).await?;
        f.write_all_buf(&mut res.bytes().await?).await?;
    }
    println!("{}/{}", count, len);
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn champion_t() {
        tokio_test::block_on(champion()).unwrap();
    }

    #[test]
    fn rune_t() {
        tokio_test::block_on(rune()).unwrap();
    }

    #[test]
    fn item_t() {
        tokio_test::block_on(item()).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    debug!("debug test");
    error!("error test");

    Ok(())
}
