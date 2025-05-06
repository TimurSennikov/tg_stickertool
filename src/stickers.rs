pub mod stickers {
    use teloxide::prelude::*;
    use teloxide::types::*;

    use opencv::{imgcodecs, prelude::*};

    use crate::converter::converter::Converter;

    trait CreatePack<T> {
        async fn new_pack(
            bot: &Bot,
            title: &str,
            stickers: &T,
            ownerid: u64,
        ) -> Result<StickerPack, String>;
    }

    trait StickerManager<T> {
        fn sticker_from(img: T) -> Result<InputSticker, &'static str>;
        async fn add_sticker(
            bot: &Bot,
            pack: &str,
            sticker: T,
            ownerid: u64,
        ) -> Result<&'static str, String>;
    }

    pub struct StickerPack {
        pub n: u32,
        pub url: String,
    }

    pub struct Stickers {}

    impl StickerManager<&str> for Stickers {
        fn sticker_from(img: &str) -> Result<InputSticker, &'static str> {
            let img = imgcodecs::imread(img, imgcodecs::IMREAD_COLOR).unwrap();
            if img.empty() {
                return Err("image is empty!");
            }

            if let Ok(r) = Converter::crop_for_tg(&img) {
                let img = Converter::prepare_for_tg(&r);
                return Ok(InputSticker {
                    sticker: InputFile::memory(img),
                    format: StickerFormat::Static,
                    emoji_list: vec!["💤".to_string()],
                    mask_position: None,
                    keywords: vec!["sex".to_string()],
                });
            } else {
                return Err("Error!");
            }
        }

        async fn add_sticker(
            bot: &Bot,
            pack: &str,
            sticker: &str,
            ownerid: u64,
        ) -> Result<&'static str, String> {
            if let Ok(stick) = Self::sticker_from(sticker) {
                if let Err(e) = bot.add_sticker_to_set(UserId(ownerid), pack, stick).await {
                    return Err(format!("Error: {}", e));
                }
            }

            Ok("Ok")
        }
    }

    impl StickerManager<&Mat> for Stickers {
        fn sticker_from(img: &Mat) -> Result<InputSticker, &'static str> {
            let prepared = Converter::prepare_for_tg(img);
            Ok(InputSticker {
                sticker: InputFile::memory(prepared),
                format: StickerFormat::Static,
                emoji_list: vec!["💤".to_string()],
                mask_position: None,
                keywords: vec!["sex".to_string()],
            })
        }

        async fn add_sticker(
            bot: &Bot,
            pack: &str,
            sticker: &Mat,
            ownerid: u64,
        ) -> Result<&'static str, String> {
            if let Ok(stick) = Self::sticker_from(sticker) {
                if let Err(e) = bot.add_sticker_to_set(UserId(ownerid), pack, stick).await {
                    return Err(format!("Error: {}", e));
                }
            }

            Ok("Ok")
        }
    }

    impl CreatePack<Vec<Mat>> for Stickers {
        async fn new_pack(
            bot: &Bot,
            title: &str,
            stickers: &Vec<Mat>,
            ownerid: u64,
        ) -> Result<StickerPack, String> {
            let me = bot.get_me().await.unwrap();
            let username = me.username.as_ref().unwrap();

            let title = format!("{} by {}", title, username);
            let name = format!(
                "stick{:?}_by_{}",
                (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()),
                username
            );

            let mut stickers_n: Vec<InputSticker> = vec![];
            stickers_n.push(Self::sticker_from(&stickers[0]).unwrap());

            if let Err(e) = bot
                .create_new_sticker_set(UserId(ownerid), &name, &title, stickers_n)
                .await
            {
                return Err(format!("Error creating pack: {}", e));
            }

            let mut n = 1;

            for sticker in stickers.into_iter().skip(1) {
                println!("Adding sticker number {}...", n + 1);

                if let Err(e) = Self::add_sticker(bot, &name, sticker, ownerid).await {
                    println!("Error adding sticker: {}", e);
                    continue;
                }

                n += 1;
            }

            if n != 25 {
                return Err(format!(
                    "Invalid stickers number! Expected {} but got only {}",
                    25, n
                ));
            }

            Ok(StickerPack {
                n,
                url: String::from(format!("https://t.me/addstickers/{}", name)),
            })
        }
    }

    impl Stickers {
        pub async fn create_trap_pack(
            bot: &Bot,
            title: &str,
            stickers: &Vec<Mat>,
            ownerid: u64,
        ) -> Result<StickerPack, String> {
            Self::new_pack(bot, title, stickers, ownerid).await
        }

        pub async fn create_packs(
            bot: &Bot,
            title: &str,
            stickerdir: &str,
            ownerid: u64,
        ) -> Result<Vec<String>, &'static str> {
            let mut last = 0;
            let mut urls: Vec<String> = vec![];

            loop {
                if let Ok(f) = Self::create_pack(
                    bot,
                    &format!("{}{}", title, last),
                    stickerdir,
                    ownerid,
                    Some(last),
                )
                .await
                {
                    last += f.n;
                    println!("Done! Download one of your packs at {}", f.url);
                    urls.push(f.url);
                } else {
                    break;
                }
            }

            if last <= 0 {
                return Err("Didn`t create sticker packs.");
            } else {
                return Ok(urls);
            }
        }

        pub async fn create_pack(
            bot: &Bot,
            title: &str,
            stickerdir: &str,
            ownerid: u64,
            startpos: Option<u32>,
        ) -> Result<StickerPack, &'static str> {
            let me = bot.get_me().await.unwrap();
            let username = me.username.as_ref().unwrap();

            let title = format!("{} by {}", title, username);
            let name = format!(
                "stick{:?}_by_{}",
                (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()),
                username
            );

            let mut stickers: Vec<InputSticker> = vec![];
            let entries = std::fs::read_dir(stickerdir).unwrap();
            let mut entries_vec: Vec<String> = vec![];

            for entry in entries {
                let p = entry.unwrap().path();
                entries_vec.push(p.to_str().unwrap().to_string());
            }

            loop {
                if entries_vec.len() <= 0 {
                    eprintln!("Out of files, failed to add sticker pack.");
                    return Err("Error creating stickerpack!");
                }

                let f = entries_vec.pop().unwrap();
                if let Ok(f) = Self::sticker_from(&f[..]) {
                    stickers.push(f);
                    break;
                } else {
                    eprintln!("Failed to add first sticker, trying next one...");
                }
            }

            if let Err(_) = bot
                .create_new_sticker_set(UserId(ownerid), &name, &title, stickers)
                .await
            {
                return Err("Error creating pack!");
            }

            let e_vec_iter = if let Some(p) = startpos {
                entries_vec.into_iter().skip(p.try_into().unwrap())
            } else {
                entries_vec.into_iter().skip(0)
            };

            let mut added = 0;
            for (i, sticker) in e_vec_iter.enumerate() {
                if i >= 120 {
                    break;
                }

                println!("Adding sticker {} to pack {}", sticker, name);
                let mut retries: u32 = 5;
                loop {
                    if let Ok(_) = Self::add_sticker(bot, &name, &sticker[..], ownerid).await {
                        break;
                    } else if retries <= 0 {
                        break;
                    }

                    retries -= 1;
                    eprintln!("Error! Retrying ({} times remaining)...", retries);
                }

                added += 1;
            }

            Ok(StickerPack {
                n: added,
                url: format!("https://t.me/addstickers/{}", name),
            })
        }
    }
}
