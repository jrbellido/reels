extern crate clap;

use crate::cli::arguments::arguments;

mod arguments;

mod command_create;
mod command_delete;
mod command_init;
mod command_list;
mod command_mount;
mod command_recover;

pub fn cli() {
    match arguments().subcommand() {
        ("init", Some(m)) => command_init::execute(m.value_of("PATH").unwrap()),
        ("create", Some(m)) => command_create::execute(
            m.value_of("repo").unwrap(),
            m.value_of("PATH").unwrap(),
            m.value_of("chroot"),
        ),
        ("delete", Some(m)) => {
            command_delete::execute(m.value_of("repo").unwrap(), m.value_of("SNAPSHOT").unwrap())
        }
        ("recover", Some(m)) => command_recover::execute(
            m.value_of("repo").unwrap(),
            m.value_of("SNAPSHOT").unwrap(),
            m.value_of("output_dir").unwrap(),
        ),
        ("list", Some(m)) => {
            command_list::execute(m.value_of("repo").unwrap(), m.value_of("OBJECT").unwrap())
        }
        ("mount", Some(m)) => {
            command_mount::execute(m.value_of("repo").unwrap(), m.value_of("TARGET").unwrap())
        }
        _ => (),
    }
}
