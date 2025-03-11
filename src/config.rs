use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub repository: String,
}

impl Config {
    fn new(repo: String) -> Self {
        Self { repository: repo }
    }

    fn new_from_user() -> anyhow::Result<Self> {
        let repo =
            inquire::prompt_text("Enter the SDP repo to track (ex. cs481-ekh/s25-team-name):")?;

        Ok(Self::new(repo))
    }
}

pub fn load_config() -> anyhow::Result<Config> {
    let path = dirs::config_dir()
        .ok_or(anyhow::anyhow!(
            "Couldn't identify user configuration directory"
        ))?
        .join("sdp-task-timer")
        .join("config.toml");

    let contents_result = fs::read_to_string(&path);

    let read_cfg = match contents_result {
        Ok(contents) => toml::from_str(&contents)?,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                let new_config = Config::new_from_user()?;
                fs::create_dir_all(path.parent().ok_or(anyhow::anyhow!(
                    "Couldn't get parent path of config directory"
                ))?)?;
                fs::write(&path, toml::to_string_pretty(&new_config)?)?;

                new_config
            }
            _ => return Err(e.into()),
        },
    };

    Ok(read_cfg)
}
