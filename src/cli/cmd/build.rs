use std::path::PathBuf;

use crate::build::cmd::{BuildCommand, HtmlMetadata};
use crate::libs::error::result::ScrapResult;

use crate::cli::scrap_config::ScrapConfig;

pub fn run() -> ScrapResult<()> {
    let scraps_dir_path = PathBuf::from("scraps");
    let static_dir_path = PathBuf::from("static");
    let public_dir_path = PathBuf::from("public");
    let command = BuildCommand::new(&scraps_dir_path, &static_dir_path, &public_dir_path);

    let config = ScrapConfig::new()?;
    let timezone = config.timezone.unwrap_or(chrono_tz::UTC);
    let html_metadata = &HtmlMetadata {
        title: config.title,
        description: config.description,
        favicon: config.favicon,
    };
    command.run(&timezone, &html_metadata)
}
