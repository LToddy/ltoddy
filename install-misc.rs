use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

#[derive(Parser)]
pub struct Args {
    #[clap(short = 'f', long = "force")]
    force: bool,
}

pub fn main() -> Result<()> {
    init_logger();
    let args: Args = Args::parse();
    let home_dir = PathBuf::from(env!("HOME"));

    let miscellaneous = vec![
        ("misc/hushlogin", home_dir.join(".hushlogin")),
        ("misc/alacritty.yml", home_dir.join(".config/alacritty/alacritty.yml")),
        ("misc/vimrc", home_dir.join(".vimrc")),
        ("misc/morning", home_dir.join(".local/bin/morning")),
        ("misc/zshrc", home_dir.join(".zshrc")),
    ];
    info!("start install misc files");
    for (from, to) in miscellaneous {
        try_copy_file(args.force, from, to)?;
    }
    Ok(())
}

pub fn init_logger() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .unwrap();
}

pub fn try_copy_file<A, B>(force: bool, from: A, to: B) -> Result<()>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let (from, to) = (from.as_ref(), to.as_ref());
    let from =
        fs::canonicalize(from).with_context(|| format!("wrong file path: `{}`", from.display()))?;
    if force || !to.exists() {
        info!("copying `{}` -> `{}`", from.display(), to.display());
        fs::copy(from, to)?;
    }
    Ok(())
}
