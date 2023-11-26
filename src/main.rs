mod fs_utils;
mod lang;
mod libio;

use std::{fs::create_dir, path::PathBuf};

use {
    anyhow::{bail, Result},
    clap::Parser,
    either::{Left, Right},
    semver::Version,
    serde::Deserialize,
    termcolor::ColorChoice,
};

use lang::rust::crates_from_cargo_file;
use termcolor::{Color, ColorSpec, WriteColor};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "File to have it's dependencies checked.")]
    file: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    librariesio: libio::LibIO,
}

fn clean_ver_str(ver_str: &mut String) -> String {
    ver_str.as_mut().replace("=", "")
}

fn main() -> Result<()> {
    let argv = Args::parse();

    // retrieve api key
    let mut cfg_path = dirs::data_local_dir().unwrap();
    cfg_path.push("chkdeps");
    if !cfg_path.exists() {
        create_dir(&cfg_path)?;
    }
    cfg_path.push("config.toml");
    if !cfg_path.exists() {
        bail!("no libraries.io API key set at {cfg_path:?}");
    }

    let cfg: Config = toml::from_str(&fs_utils::file_contents(cfg_path)?)?;

    // look for applicable files
    let cargo = crates_from_cargo_file(argv.file)?;

    // get library versions
    let stdout = termcolor::BufferWriter::stdout(ColorChoice::Always);
    let mut stdbuf = stdout.buffer();
    for (name, dep) in cargo.dependencies {
        let cver = match dep.ver_or_info {
            Left(mut ver) => Version::parse(&clean_ver_str(&mut ver))?,
            Right(mut info) => Version::parse(&clean_ver_str(&mut info.version))?,
        };
        let lib = libio::get_lib("Cargo", &name, &cfg.librariesio.api_key)?;
        let lver = Version::parse(&lib.latest_stable_release_number)?;

        // print local vs remote comparison
        use std::io::Write;
        if lver > cver {
            stdbuf.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            write!(&mut stdbuf, "❌🦀 {name}:")?;

            stdbuf.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(&mut stdbuf, "{cver} ")?;
        } else {
            stdbuf.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            write!(&mut stdbuf, "✔ 🦀 {name}: ")?;

            stdbuf.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(&mut stdbuf, "{cver} ")?;
        }
        stdbuf.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut stdbuf, "- ")?;

        stdbuf.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdbuf, "{lver}")?;

        stdbuf.flush()?;
        stdout.print(&stdbuf)?;
        stdbuf.clear();
    }

    Ok(())
}
