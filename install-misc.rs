use std::fs::canonicalize;
use std::io;
use std::path::{Path, PathBuf};

use futures::{stream, StreamExt};
use log::LevelFilter::Debug;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use tokio::fs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
pub async fn main() {
    TermLogger::init(Debug, Default::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    if let Err(e) = run().await {
        eprintln!("{e}");
    }
}

pub async fn run() -> Result<()> {
    let home_dir = PathBuf::from(env!("HOME"));

    let miscellaneous = vec![
        ("misc/hushlogin", home_dir.join(".hushlogin")),
        ("misc/alacritty.yml", home_dir.join(".config/alacritty/alacritty.yml")),
        ("misc/vimrc", home_dir.join(".vimrc")),
    ];
    miscellaneous
        .iter()
        .map(|(from, to)| copy_if_not_exist(from, to))
        .collect::<stream::FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await;
    Ok(())
}

pub async fn copy_if_not_exist(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    let from = canonicalize(from)?;
    let to = to.as_ref();
    if !to.exists() {
        log::info!("copying `{}` -> `{}`", from.display(), to.display());
        fs::copy(from, to).await?;
    }
    Ok(())
}
