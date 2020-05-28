use anyhow::{anyhow, Result};
use structopt::clap::AppSettings::{ColorNever, NoBinaryName, TrailingVarArg};
use structopt::StructOpt;

#[derive(StructOpt, Default, Debug)]
#[structopt(setting = NoBinaryName, setting = ColorNever, setting = TrailingVarArg)]
pub struct Opt {
    /// Whether to use CROM Canary (CROM 2).
    #[structopt(short = "2", long)]
    pub canary: bool,

    /// The search query to use.
    #[structopt(allow_hyphen_values = true)]
    pub query: Vec<String>,
}

pub fn parse(input: &str) -> Result<Opt> {
    let mut split = input.splitn(2, "-- ");
    let opts = split
        .next()
        .ok_or_else(|| anyhow!("Missing options!"))?
        .trim();
    let mut shlexed = if let Some(x) = shlex::split(opts) {
        x
    } else {
        return Ok(Opt {
            canary: false,
            query: vec![input.to_string()],
        });
    };
    if let Some(remainder) = split.next() {
        shlexed.push("--".into());
        shlexed.push(remainder.trim().into());
    }
    println!("[Parse] Arguments: {:?}", shlexed);
    Ok(Opt::from_iter_safe(shlexed.into_iter())?)
}
