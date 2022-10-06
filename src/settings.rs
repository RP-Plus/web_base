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

fn get_settings() -> &'static config::Config {

    &CACHED_SETTINGS

}


pub fn get_setting(config_option: &str) -> String {

    let settings = get_settings();
    settings.get_str(config_option).unwrap()

}