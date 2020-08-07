use super::sanitize::CleanHtml;
use elefren::entities::account::Account;
use elefren::entities::event::Event;
use elefren::entities::notification::{Notification, NotificationType};
use elefren::entities::status::Status;
use elefren::helpers::{cli, toml};
use elefren::prelude::*;
use itertools::Itertools;
use lazy_format::lazy_format;
use log::*;
use std::error::Error;
use std::iter;
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
        .client_name("mastocrom")
        .scopes(Scopes::all())
        .website("https://github.com/leo60228/mastocrom")
        .build()?;
    let mastodon = cli::authenticate(registration)?;

    // Save app data for using on the next run.
    toml::to_file(&*mastodon, "mastodon-data.toml")?;

    Ok(mastodon)
}

async fn respond_to(
    client: &crom::Client<http_client::native::NativeClient>,
    search: &str,
) -> Result<String, Box<dyn Error>> {
    debug!("Got search {:?}", search);
    let pages = client
        .search(
            search,
            Some(vec!["http://scp-wiki.wikidot.com".to_string()]),
        )
        .await?;
    if let Some(page) = pages.get(0) {
        if let Some(crom::PageAlternateTitle { title, .. }) = page.alternate_titles.get(0) {
            Ok(format!("{} - {}", title, page.url))
        } else if let Some(title) = page.wikidot_info.as_ref().and_then(|x| x.title.as_ref()) {
            Ok(format!("{} - {}", title, page.url))
        } else {
            Ok(page.url.to_string())
        }
    } else {
        Ok("No results.".to_string())
    }
}

fn reply_mentions(status: &Status, account: &Account) -> String {
    let author = &status.account.acct;
    iter::once(author)
        .chain(
            status
                .mentions
                .iter()
                .filter(|x| x.url != account.url)
                .map(|x| &x.acct),
        )
        .map(|x| lazy_format!("@{}", x))
        .join(" ")
}

pub async fn start() -> Result<!, Box<dyn Error>> {
    let mastodon = get_mastodon_data()?;

    let acc = mastodon.verify_credentials()?;
    info!("Connected as {}", acc.username);

    let at = format!("@{}", acc.username);

    let crom = crom::Client::new();

    loop {
        trace!("Polling...");
        let mut last_status = None::<String>;
        for notif in mastodon.streaming_user()? {
            if let Event::Notification(Notification {
                notification_type: NotificationType::Mention,
                status: Some(status),
                ..
            }) = notif
            {
                if matches!(last_status, Some(ref x) if x == &status.id) {
                    continue;
                }
                info!("Received query!");
                let cleaned = CleanHtml(&status.content).to_string();
                for query in cleaned.split(&at).skip(1) {
                    let query = query.split("\n").next().unwrap_or("").trim();
                    let result = respond_to(&crom, query).await?;
                    let reply = format!("{} {}", reply_mentions(&status, &acc), result);
                    let reply_status = StatusBuilder::new()
                        .status(reply.trim_start())
                        .in_reply_to(&status.id)
                        .build()?;
                    mastodon.new_status(reply_status)?;
                }
                last_status = Some(status.id);
            }
        }
        mastodon.clear_notifications()?;
        trace!("Sleeping...");
        thread::sleep(Duration::from_secs(15));
    }
}
