extern crate argparse;

use std::process::exit;
use argparse::{ArgumentParser, Store, StoreConst};
use crate::actions::actions::{
    add_action,
    add_action_at_index,
    list_actions,
    rm_action,
    rm_all,
    ActionResult
};

pub mod actions;
mod config;

#[derive(Clone, Eq, PartialEq, Debug)]
enum AppMode {
    Add,
    AddToStart,
    Rm,
    RmFirst,
    RmLast,
    RmAll,
    ListRecords,
}

#[derive(Clone, Debug)]
struct AppOptions {
    mode: AppMode,
    action: String,
    position: i64,
}

fn parse_args() -> AppOptions {
    let mut opts: AppOptions = AppOptions {
        mode: AppMode::Add,
        action: "".to_string(),
        position: -1,
    };

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Simple CLI Utility to manage a to-do list.");

        ap.refer(&mut opts.mode)
            .add_option(
                &["-a", "--add"],
                StoreConst(AppMode::Add),
                "Add to-do list record.",
            )
            .add_option(
                &["-s", "--insert"],
                StoreConst(AppMode::AddToStart),
                "Add record to the start of the to-do list.",
            )
            .add_option(
                &["-r", "--remove"],
                StoreConst(AppMode::Rm),
                "Remove record.",
            )
            .add_option(
                &["-t", "--last"],
                StoreConst(AppMode::RmLast),
                "Remove the latest record.",
            )
            .add_option(
                &["-y", "--first"],
                StoreConst(AppMode::RmFirst),
                "Remove the first record.",
            )
            .add_option(
                &["-l", "--list"],
                StoreConst(AppMode::ListRecords),
                "List records.",
            )
            .add_option(
                &["-C", "--clear"],
                StoreConst(AppMode::RmAll),
                "Remove all records.",
            );

        ap.refer(&mut opts.action).add_argument(
            "record_or_position",
            Store,
            "To-do list record (Add) or position (Remove).",
        );

        ap.refer(&mut opts.position).add_argument(
            "position",
            Store,
            "To-do list record position, may be used with -a",
        );

        ap.parse_args_or_exit();
    }

    if (
        opts.mode == AppMode::Add ||
            opts.mode == AppMode::Rm ||
            opts.mode == AppMode::AddToStart
    ) && opts.action == "" {
        opts.mode = AppMode::ListRecords;
    }

    opts
}


fn main() {
    let opts = parse_args();

    let result: ActionResult = match opts.mode {
        AppMode::ListRecords => {
            list_actions()
        }
        AppMode::Add => {
            if opts.position == -1 {
                add_action(opts.action)
            } else {
                add_action_at_index(opts.action, opts.position - 1)
            }
        }
        AppMode::AddToStart => {
            add_action_at_index(opts.action, 0)
        }
        AppMode::Rm => {
            match opts.action.parse() {
                Ok(val) => {
                    if val < 1 {
                        println!(
                            "Number of item to remove cannot be lesser than 1."
                        );
                        ActionResult::UnableToRemoveAction
                    } else {
                        rm_action(val)
                    }
                }
                Err(_) => {
                    println!("Expecting parameter number of item to remove.");
                    exit(1);
                }
            }
        }
        AppMode::RmFirst => {
            rm_action(1)
        }
        AppMode::RmLast => {
            rm_action(-1)
        }
        AppMode::RmAll => {
            rm_all()
        }
    };

    if result == ActionResult::Ok {
        exit(0);
    } else {
        exit(1);
    }
}
