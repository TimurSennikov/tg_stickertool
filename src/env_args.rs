pub mod env_args {
    pub enum Mode {
        FromDir(Options),
        StickerTrap(Options),
        PollMode,
    }

    pub struct Options {
        pub pack_name: String,
        pub path: String,
        pub user_id: String,
    }

    pub fn get_options() -> Result<Mode, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 5 && args.len() != 2 {
            return Err("run 'stickertool help'.");
        }

        match args.len() {
            2 => match args[1].as_str() {
                "help" => {
                    println!("Telegram sticker creatinon tool.\n
TELOXIDE_TOKEN=token stickertool (stickerpack name) (sticker / stickers path) (stickerpack owner id) (mode)\n
stickerpack name is any valid string 1-64 characters long.\n
sticker/stickers path is path to sticker or sticker directory (depending on mode argument)\n
stickerpack owner id - id which will own the pack.\n
mode - can be from_dir or sticker_trap. In first case sticker/stickers path should contain path to directory with images. In the secode case it should be a path to a single image file.\n
TELOXIDE_TOKEN=token stickertool poll - run in the polling mode using the token as the bot token. Send any photo to the (token) bot and it will send you a trap stickerpack.\n

stickertool help - display this text.\n
                                        ");
                    std::process::exit(0);
                }
                "poll" => return Ok(Mode::PollMode),
                _ => panic!("Invalid parameter provided!"),
            },

            5 => {
                let options = Options {
                    pack_name: args[1].clone(),
                    path: args[2].clone(),
                    user_id: args[3].clone(),
                };
                match args[4].as_str() {
                    "from_dir" => return Ok(Mode::FromDir(options)),
                    "sticker_trap" => return Ok(Mode::StickerTrap(options)),
                    _ => return Err("Can`t identify mode!"),
                }
            }

            _ => panic!("Invalid arguments provided!"),
        }
    }
}
