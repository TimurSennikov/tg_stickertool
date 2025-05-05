pub mod env_args {
    pub enum Mode {
        FromDir(Options),
        StickerTrap(Options)
    }

    pub struct Options {
        pub pack_name: String,
        pub path: String,
        pub user_id: String
    }

    pub fn get_options() -> Result<Mode, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 5 {return Err("Usage example: command (stickerpack name) (sticker / stickers path) (stickerpack owner id) (mode)");}

        let options = Options{pack_name: args[1].clone(), path: args[2].clone(), user_id: args[3].clone()};

        if args[4] == "from_dir" {return Ok(Mode::FromDir(options));}
        else if args[4] == "sticker_trap" {return Ok(Mode::StickerTrap(options));}
        else {return Err("Can`t identify mode!");}
    }
}
