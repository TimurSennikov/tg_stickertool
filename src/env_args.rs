pub mod env_args {
    pub enum Mode {
        FromDir(Options),
        StickerTrap(Options),
        PollMode
    }

    pub struct Options {
        pub pack_name: String,
        pub path: String,
        pub user_id: String
    }

    pub fn get_options() -> Result<Mode, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 5 && args.len() != 2 {return Err("Usage example: command (stickerpack name) (sticker / stickers path) (stickerpack owner id) (mode)");}

        match args.len() {
            2 => {
                match args[1].as_str(){
                    "help" => {println!("command (stickerpack name) (sticker / stickers path) (stickerpack owner id) (mode)"); std::process::exit(0);},
                    "poll" => return Ok(Mode::PollMode),
                    _ => panic!("Invalid parameter provided!")
                }
            },

            5 => {
                let options = Options{pack_name: args[1].clone(), path: args[2].clone(), user_id: args[3].clone()};
                match args[4].as_str(){
                    "from_dir" => return Ok(Mode::FromDir(options)),
                    "sticker_trap" => return Ok(Mode::StickerTrap(options)),
                    _ => return Err("Can`t identify mode!")}
            },

            _ => panic!("Invalid arguments provided!")
        }
    }
}