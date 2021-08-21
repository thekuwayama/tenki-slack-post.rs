#[macro_use]
extern crate clap;

use anyhow::Result;
use chrono::Local;
use clap::{App, Arg};
use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::Cursor;
use tempfile::tempdir;
use tenki_slack_post::{slack, tenki};

const TEMP_TENKI_IMAGE: &str = "tenki.jpg";

fn main() -> Result<()> {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("oauth-token")
                .help("Slack OAuth Token")
                .required(true),
        )
        .arg(
            Arg::with_name("channel")
                .help("Slack Channel Name")
                .required(true),
        );
    let matches = cli.get_matches();
    let oauth_token = matches
        .value_of("oauth-token")
        .expect("Falid: not specify Slack OAuth Token");
    let channel = matches
        .value_of("channel")
        .expect("Falid: not specify Slack Channel Name");

    // Get image from tenki.jp
    let now = Local::now();
    let img = tenki::get_tenki_image(now)?;

    // WebP => JPEG tempfile
    let img = ImageReader::new(Cursor::new(img))
        .with_guessed_format()?
        .decode()?;
    let dir = tempdir()?;
    let file_path = dir.path().join(TEMP_TENKI_IMAGE);
    let file = File::create(file_path.clone())?;
    img.save_with_format(file_path.as_path(), image::ImageFormat::Jpeg)?;

    // Post image to Slack
    slack::post_image_to_slack(channel, oauth_token, file_path.as_path())?;
    drop(file);
    dir.close()?;
    Ok(())
}
