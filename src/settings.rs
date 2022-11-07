use std::path::Path;


lazy_static! {

    static ref CACHED_SETTINGS: config::Config = {

        let mut settings = config::Config::default();

        if Path::new("LocalSettings.toml").exists() {

            settings.merge(config::File::with_name("LocalSettings")).unwrap();
            settings

        } else {

            settings.merge(config::File::with_name("Settings")).unwrap();
            settings

        }

    };

}

fn get_all() -> &'static config::Config {

    &CACHED_SETTINGS

}


pub fn get(config_option: &str) -> String {

    CACHED_SETTINGS.get_str(config_option).unwrap()

}