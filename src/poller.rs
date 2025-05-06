pub mod poller {
    use teloxide::{
        net::Download,
        prelude::*,
        types::{MediaKind, MessageKind},
    };
    use tokio::fs;

    use crate::converter::converter::Converter;
    use crate::stickers::stickers::Stickers;

    pub struct BotPoller {}

    impl BotPoller {
        pub async fn poll(bot: Bot) {
            teloxide::repl(bot, async |bot: Bot, message: Message| {
                if let MessageKind::Common(ref msgcommon) = message.kind {
                    if let MediaKind::Photo(m) = &msgcommon.media_kind {
                        if let None = message.from {
                            bot.send_message(message.chat.id, "Cannot get your id.")
                                .await
                                .expect("Failed to send message!");
                            return Ok(());
                        }

                        let img_id = &m.photo[m.photo.len() - 1].file.id;

                        let file = bot.get_file(img_id).await.unwrap();

                        let time_now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let fname = format!("/tmp/{}.png", time_now);

                        let dst = fs::File::create(&fname).await;
                        if let Err(ef) = dst {
                            eprintln!("{}", ef);

                            if let Err(e) = bot
                                .send_message(message.chat.id, "Failed to create temporary file!")
                                .await
                            {
                                eprintln!("Error sending message: {}", e);
                            }

                            return Ok(());
                        }
                        let mut dst = dst.unwrap();

                        bot.send_message(message.chat.id, "Ок ща сделаю")
                            .await
                            .expect("Failed to send message!");
                        if let Err(_) = bot.download_file(&file.path, &mut dst).await {
                            bot.send_message(message.chat.id, "Failed to download photo.")
                                .await
                                .expect("Failed to send message!");
                        }
                        let img_orig =
                            opencv::imgcodecs::imread(&fname, opencv::imgcodecs::IMREAD_COLOR)
                                .expect("Failed to read image!");

                        bot.send_message(message.chat.id, "Splitting photo to 25 pieces.")
                            .await
                            .expect("Failed to send message!");
                        let stickers = Converter::split_for_tg(&img_orig).unwrap();
                        bot.send_message(
                            message.chat.id,
                            "OK! Collecting pack...",
                        )
                        .await
                        .expect("Failed to send message!");

                        let r = Stickers::create_trap_pack(
                            &bot,
                            ">._.>",
                            &stickers,
                            message.from.unwrap().id.0,
                        )
                        .await
                        .expect("Failed to create stickerpack!");

                        bot.send_message(
                            message.chat.id,
                            format!("Here is your pack: {}", r.url),
                        )
                        .await
                        .expect("Failed to send message!");
                    }
                }

                Ok(())
            })
            .await;
        }
    }
}
