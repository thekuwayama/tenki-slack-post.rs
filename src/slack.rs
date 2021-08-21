use anyhow::Result;
use reqwest::blocking::multipart;
use reqwest::header::AUTHORIZATION;
use std::path::Path;

pub fn post_image_to_slack(channel: &str, oauth_token: &str, file_path: &Path) -> Result<()> {
    let url = "https://slack.com/api/files.upload";
    let form = multipart::Form::new()
        .text("channels", channel.to_owned())
        .file("file", file_path)?;

    let cli = reqwest::blocking::Client::new();
    cli.post(url)
        .header(AUTHORIZATION, format!("Bearer {}", oauth_token))
        .multipart(form)
        .send()?;

    Ok(())
}
