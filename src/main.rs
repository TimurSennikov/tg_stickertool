use teloxide::prelude::*;

use opencvtest::stickers::stickers::Stickers;
use opencvtest::converter::converter::Converter;
use opencvtest::poller::poller::BotPoller;
use opencvtest::env_args::{env_args, env_args::Mode};

use opencv::imgcodecs;

#[tokio::main]
async fn main() {
    let args = env_args::get_options();

    match args {
        Ok(mode) => {
            let bot = Bot::from_env();

            match mode {
                Mode::FromDir(a) => {
                    match Stickers::create_packs(&bot, &a.pack_name, &a.path, a.user_id.parse().unwrap()).await {
                        Ok(_) => println!("Done."),
                        Err(e) => eprintln!("Error creating stickerpacks: {}", e)
                    }
                },

                Mode::StickerTrap(a) => {
                    match Stickers::create_trap_pack(&bot, &a.pack_name, &Converter::split_for_tg(&imgcodecs::imread(&a.path, imgcodecs::IMREAD_COLOR).unwrap()).unwrap(), a.user_id.parse().unwrap()).await {
                        Ok(pack) => println!("Done! Get your sticker trap at {}", pack.url),
                        Err(e) => eprintln!("Error creating stickerpack: {}", e)
                    }
                },

                Mode::PollMode => {BotPoller::poll(bot).await; std::process::exit(0);}
            }
        },

        Err(e) => {
            eprintln!("{}", e);
        }
    }
}