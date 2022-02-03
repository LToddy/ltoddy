use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use anyhow::Context;
use futures::{stream, StreamExt};
use tokio::fs;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    init_logger();
    let home_dir = PathBuf::from(env!("HOME"));

    let miscellaneous = vec![
        ("misc/hushlogin", home_dir.join(".hushlogin")),
        ("misc/alacritty.yml", home_dir.join(".config/alacritty/alacritty.yml")),
        ("misc/vimrc", home_dir.join(".vimrc")),
    ];
    miscellaneous
        .iter()
        .map(|(from, to)| try_copy_file(from, to))
        .collect::<stream::FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await;
    Ok(())
}

pub fn init_logger() {
    use log::LevelFilter;
    use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .unwrap();
}

pub async fn try_copy_file<A, B>(from: A, to: B) -> anyhow::Result<()>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let (from, to) = (from.as_ref(), to.as_ref());
    let from =
        canonicalize(from).with_context(|| format!("wrong file path: `{}`", from.display()))?;
    if !to.exists() {
        log::info!("copying `{}` -> `{}`", from.display(), to.display());
        fs::copy(from, to).await?;
    }
    Ok(())
}
