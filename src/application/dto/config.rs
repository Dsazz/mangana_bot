use std::env;
use std::borrow::Borrow;
use teloxide::types::InputFile;

lazy_static! {
    pub static ref CONFIG_REF: Config = {
        Config::new()
    };
}

#[allow(clippy::module_name_repetitions)]
pub fn get_config() -> &'static CONFIG_REF {
    CONFIG_REF.borrow()
}

pub struct Config {
    admin_id: i64,
    preloader_url: String,
}
impl Config {
    pub(crate) fn new() -> Config {
        let admin_id = env::var("ADMIN_ID").expect("ADMIN_ID must be set").parse::<i64>().unwrap();
        let preloader_url = "https://media.giphy.com/media/TkosWhXLczmjv16kn2/giphy.gif".to_string();

        Config { admin_id, preloader_url }
    }

    pub fn preloader_animation(&self) -> InputFile {
        InputFile::Url(self.preloader_url.to_string())
    }
    pub fn is_user_admin(&self, user_id: i64) -> bool {
        self.admin_id == user_id
    }
}