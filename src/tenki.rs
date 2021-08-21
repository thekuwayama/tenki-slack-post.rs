use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::{DateTime, Local};
use reqwest::StatusCode;

pub fn get_tenki_image(dt: DateTime<Local>) -> Result<bytes::Bytes> {
    for i in 0..3 {
        let url = format_tenki_url(dt, i);
        let cli = reqwest::blocking::Client::new();
        let res = cli.get(url).send()?;
        if res.status() == StatusCode::OK {
            return Ok(res.bytes()?);
        }
    }

    Err(anyhow!("Failed to get tenki.jp image"))
}

fn format_tenki_url(dt: DateTime<Local>, i: u32) -> String {
    let dt = gen_target_datetime(dt, i);
    let fmt =
        "https://imageflux.tenki.jp/large/static-images/radar/%Y/%m/%d/%H/%M/00/pref-17-large.jpg";
    dt.format(fmt).to_string()
}

fn gen_target_datetime(dt: DateTime<Local>, i: u32) -> DateTime<Local> {
    let i: i64 = From::from(i);
    // 5 minutes interval
    Local.timestamp(dt.timestamp() - dt.timestamp() % 300 - 300 * i, 0)
}
