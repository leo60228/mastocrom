use super::crom;
use super::sanitize::CleanHtml;
use elefren::entities::notification::{Notification, NotificationType};
use elefren::helpers::{cli, toml};
use elefren::prelude::*;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn get_mastodon_data() -> Result<Mastodon, Box<dyn Error>> {
    if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Ok(Mastodon::from(data))
    } else {
        register()
    }
}

fn register() -> Result<Mastodon, Box<dyn Error>> {
    let website = "https://60228.dev";
    let registration = Registration::new(website.trim())
        .client_name("upd8r")
        .scopes(Scopes::all())
        .website("https://github.com/leo60228/upd8r")
        .build()?;
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}

fn respond_to(msg: &str) -> Result<String, Box<dyn Error>> {
    let query = CleanHtml(msg)
        .to_string()
        .replace("@mastocrom@60228.dev", "")
        .replace("@mastocrom", "");
    let trimmed = query.trim();
    println!("[Mastodon] Query: {}", trimmed);
    let page = crom::search(trimmed)?;
    if let Some(page) = page {
        if let Some(title) = page.title {
            Ok(format!("{} - {}", title, page.url))
        } else {
            Ok(page.url.to_string())
        }
    } else {
        Ok("No results.".to_string())
    }
}

pub fn start() -> Result<!, Box<dyn Error>> {
    let mastodon = get_mastodon_data()?;

    let acc = mastodon.verify_credentials()?;
    println!("[Mastodon] Connected as {}", acc.username);

    loop {
        println!("[Mastodon] Polling...");
        for notif in mastodon.notifications()?.items_iter() {
            println!("[Mastodon] Received notification!");
            if let Notification {
                notification_type: NotificationType::Mention,
                status: Some(status),
                ..
            } = notif
            {
                println!("[Mastodon] Received query!");
                let reply = respond_to(&status.content)?;
                let reply_status = StatusBuilder::new()
                    .status(reply)
                    .in_reply_to(status.id)
                    .build()?;
                mastodon.new_status(reply_status)?;
            }
        }
        mastodon.clear_notifications()?;
        println!("[Mastodon] Sleeping...");
        thread::sleep(Duration::from_secs(15));
    }
}
