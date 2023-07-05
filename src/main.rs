pub mod champion;
pub mod item;
pub mod profile;
pub mod rune;

use champion::Champion;
use env_logger::Env;
use item::Item;
use log::{debug, error};
use profile::Profile;
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

pub async fn profile() -> Result<(), Error> {
    let res = get("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/profile-icons.json").await?.error_for_status()?;

    let profile_list: Vec<Profile> = serde_json::from_str(&res.text().await?)?;

    // https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default /v1/profile-icons/0.jpg
    //                                                              /lol-game-data/assets /v1/profile-icons/0.jpg

    let base_url =
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";

    let mut set = JoinSet::new();

    let mut sum = 0;
    for profile in profile_list {
        let mut url = base_url.to_string();
        if let Some(path) = profile.path {
            let path = path
                .strip_prefix("/lol-game-data/assets")
                .ok_or("字段值异常")?
                .to_lowercase();
            url.push_str(&path);
            sum += 1;
            set.spawn(download(url, profile.id.unwrap()));
        }
    }

    let mut count = 0;
    while let Some(_) = set.join_next().await {
        count += 1;
        println!("{count}/{sum}");
    }

    Ok(())
}

async fn download(url: String, id: i32) -> Result<(), Error> {
    let path = std::format!("./static/profile/{}.jpg", id);
    let t = std::path::Path::new(&path);
    if t.exists() {
        if t.metadata().unwrap().len() != 0 {
            return Ok(());
        }
        tokio::fs::remove_file(t).await.expect("删除文件失败");
    };
    for i in 0..10 {
        let res = get(&url).await;
        if res.is_err() {
            println!("{}, {}/10", url, i);
            tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
            continue;
        }
        let res = res?;
        let mut f = fs::File::create(path).await?;
        f.write_all_buf(&mut res.bytes().await?).await?;

        return Ok(());
    }
    println!("{}, faild!!!!!!", url);
    Ok(())
}

// pub async fn profile() -> Result<(), Error> {
//     let res = get("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/profile-icons.json").await?.error_for_status()?;

//     let profile_list: Vec<Profile> = serde_json::from_str(&res.text().await?)?;

//     println!("{:#?}", profile_list);
//     let len = profile_list.len();

//     // https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default /v1/profile-icons/0.jpg
//     //                                                              /lol-game-data/assets /v1/profile-icons/0.jpg

//     let base_url =
//         "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default";

//     let mut count = 0;
//     for profile in profile_list {
//         let mut url = base_url.to_string();
//         if let Some(path) = profile.path {
//             let path = path
//                 .strip_prefix("/lol-game-data/assets")
//                 .ok_or("字段值异常")?
//                 .to_lowercase();
//             url.push_str(&path);

//             let res = get(url).await?.error_for_status()?;
//             if res.status().is_success() {
//                 count += 1;
//                 println!("{}/{}, {}", count, len, res.url().path());
//                 let path = std::format!("./static/profile/{}.png", profile.id.unwrap());
//                 let mut f = fs::File::create(path).await?;
//                 f.write_all_buf(&mut res.bytes().await?).await?;
//             }
//         }
//     }
//     // println!("{}/{}", count, len);
//     Ok(())
// }

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

    #[test]
    fn profile_t() {
        tokio_test::block_on(profile()).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    debug!("debug test");
    error!("error test");

    Ok(())
}
