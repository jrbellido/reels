extern crate clap;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

pub fn arguments() -> clap::ArgMatches<'static> {
    let repo_arg = Arg::with_name("repo")
        .long("repo")
        .short("r")
        .required(true)
        .takes_value(true);

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes a new repository")
                .arg(Arg::with_name("PATH").required(true)),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new snapshot")
                .arg(Arg::with_name("PATH").required(true))
                .arg(
                    Arg::with_name("chroot")
                        .long("chroot")
                        .short("c")
                        .required(false)
                        .takes_value(true),
                )
                .arg(&repo_arg),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a snapshot by id")
                .arg(Arg::with_name("SNAPSHOT").required(true))
                .arg(&repo_arg),
        )
        .subcommand(
            SubCommand::with_name("recover")
                .about("Recovers a snapshot by id")
                .arg(Arg::with_name("SNAPSHOT").required(true))
                .arg(
                    Arg::with_name("output_dir")
                        .long("output_dir")
                        .short("o")
                        .required(true)
                        .takes_value(true),
                )
                .arg(&repo_arg),
        )
        .subcommand(
            SubCommand::with_name("mount")
                .about("Mounts a repository")
                .arg(Arg::with_name("TARGET").required(true))
                .arg(&repo_arg),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Prints a list of available snapshots")
                .arg(Arg::with_name("OBJECT").required(true))
                .arg(&repo_arg),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches()
}
