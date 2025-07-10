use dirs;
use log::info;
use std::{error::Error, fs};

use crate::constants::{CONFIG_DIR, CONVERTERS_DIR};

const FILETYPES_JSON: &str = include_str!("../filetypes.json");

const CONVERTERS: &[(&str, &str)] = &[
    (
        "ffmpeg_audio",
        include_str!("../converters/ffmpeg_audio.json"),
    ),
    (
        "ffmpeg_image.json",
        include_str!("../converters/ffmpeg_image.json"),
    ),
    (
        "ffmpeg_video.json",
        include_str!("../converters/ffmpeg_video.json"),
    ),
    ("pandoc.json", include_str!("../converters/pandoc.json")),
];

pub fn setup_user_files() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let config_dir = home_dir.join(CONFIG_DIR);
    let converters_dir = config_dir.join(CONVERTERS_DIR);

    fs::create_dir_all(&config_dir)?;
    fs::create_dir_all(&converters_dir)?;

    let filetypes_path = config_dir.join("filetypes.json");
    if !filetypes_path.exists() {
        fs::write(&filetypes_path, FILETYPES_JSON)?;
        info!("Created: {}", filetypes_path.display());
    }

    for (filename, content) in CONVERTERS {
        let file_path = converters_dir.join(filename);
        if !file_path.exists() {
            fs::write(&file_path, content)?;
            info!("Created: {}", file_path.display());
        }
    }

    Ok(())
}
