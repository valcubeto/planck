#![allow(unused)]

mod clipboard;
mod common;

use common::*;

fn main() {
    let flags = init_flags();

    if flags.profile {
        todo!("profiling");
    }

    init_logs();

    if let Err(err) = load_config_dir(flags.config_dir) {
        eprintln!("failed to load config dir");
    }

    load_plugins();

    let screen = init_screen();

    // notify sigint

    // bindings

    let cb = Clipboard::new();

    // init_colorscheme();

    // crossterm::event::read()?;
}

fn init_screen() {}

fn load_plugins() {}

fn load_config_dir(dir: impl AsRef<str>) -> Result<(), Box<str>> {
    Ok(())
}

fn init_logs() {}

fn init_flags() -> Flags {
    let profile = false;
    let config_dir = Box::from("~/.config/planck");

    Flags {
        config_dir,
        profile
    }
}

pub struct Flags {
    config_dir: Box<str>,
    profile: bool
}
