use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{arg, Command};
use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

pub fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let matches = Command::new("misc").arg(arg!(-f --force "force")).get_matches();
    let force = matches.get_flag("force");
    let home_dir = std::env::home_dir().context("failed to get home directory")?;

    let miscellaneous = vec![
        ("hushlogin", include_str!("misc/hushlogin"), home_dir.join(".hushlogin")),
        ("vimrc", include_str!("misc/vimrc"), home_dir.join(".vimrc")),
        ("morning", include_str!("misc/morning"), home_dir.join(".local/bin/morning")),
        ("zshrc", include_str!("misc/zshrc"), home_dir.join(".zshrc")),
    ];

    info!("start install misc files");
    for (name, content, to) in miscellaneous {
        try_write_file(force, name, content, to)?;
    }

    info!("install misc files completed");
    Ok(())
}

pub fn try_write_file(
    force: bool,
    name: impl Into<String>,
    content: impl AsRef<str>,
    to: impl AsRef<Path>,
) -> Result<()> {
    let name = name.into();
    let content = content.as_ref();
    let to = to.as_ref();

    if force || !to.exists() {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create parent dir: `{}`", parent.display()))?;
        }
        info!("writing `{}` -> `{}`", name, to.display());
        fs::write(to, content)
            .with_context(|| format!("failed to write file: `{}`", to.display()))?;
    }
    Ok(())
}
