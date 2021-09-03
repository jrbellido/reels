extern crate clap;

use cli::cli;

mod chunk;
mod cli;
mod database;
mod hash;
mod metadata;
mod mount;
mod path;
mod profiler;
mod recovery;
mod reference;
mod repo;
mod snapshot;
mod stats;

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .write_style(env_logger::WriteStyle::Never)
        .init();

    cli();
}
