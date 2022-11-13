use crate::tui;
use clap::Command;

use crate::cache::{delete::prune_all, read::read_all_notifications};
use crate::daemon::daemonize::Daemon;
use crate::poll;

pub fn init() {
    let matches = clap::Command::new("ghostie")
        .about("manage your github notifications in terminal")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .subcommand(
            Command::new("view")
                .short_flag('V')
                .about("Open UI to manage github notifications"),
        )
        .subcommand(
            Command::new("count")
                .short_flag('C')
                .about("Query the count of unread github notifications"),
        )
        .subcommand(Command::new("start").about("Run ghostie as a background process"))
        .subcommand(Command::new("stop").about("Stop ghostie as a background process"))
        .subcommand(
            Command::new("logs")
                .short_flag('L')
                .about("Show logs from the background process"),
        )
        .subcommand(
            Command::new("prune")
                .short_flag('P')
                .about("Prune all notifications from the local cache"),
        )
        .subcommand(Command::new("clear-logs").about("Clear logs from the background process"))
        .get_matches();

    match matches.subcommand() {
        Some(("count", _)) => {
            let notifications = read_all_notifications();
            println!("{}", notifications.len())
        }
        Some(("prune", _)) => prune_all(),
        Some(("start", _)) => Daemon::daemonize(poll::start),
        Some(("stop", _)) => Daemon::stop_daemon(),
        Some(("logs", _)) => Daemon::show_logs(),
        Some(("clear-logs", _)) => Daemon::clear_logs(),
        Some(("view", _)) => {
            tui::terminal::open().unwrap();
        }

        _ => unreachable!(),
    };
}
