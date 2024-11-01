use crate::actions::actions::{AddRecordAction, AppAction, ListRecords, PerformAction, PrintHelp, RmRecordAction};

pub mod actions;

fn main() {
    run()
}

fn run() {
    let args: Vec<String> = std::env::args().collect();

    let action: AppAction;
    if args.len() < 2 {
        action = AppAction::PrintHelp(PrintHelp);
    } else {
        if args.len() < 3 {
            let mode = args[1].as_str();
            match mode {
                "-l" | "--list" => {
                    action = AppAction::ListRecords(ListRecords);
                }
                "-h" | "--help" | _ => {
                    action = AppAction::PrintHelp(PrintHelp);
                }
            }
        } else {
            let mode = args[1].as_str();
            let input = args[2].as_str();
            match mode {
                "-a" | "--add" => {
                    action = AppAction::AddRecord(
                        AddRecordAction::new(String::from(input))
                    );
                }
                "-r" | "--remove" => {
                    action = AppAction::RmRecord(
                        RmRecordAction::new(String::from(input))
                    );
                }
                _ => {
                    action = AppAction::PrintHelp(PrintHelp);
                }
            }
        }
    }

    match action {
        AppAction::PrintHelp(action) => { action.perform(); }
        AppAction::ListRecords(action) => { action.perform(); }
        AppAction::AddRecord(action) => { action.perform(); }
        AppAction::RmRecord(action) => { action.perform(); }
    }
}
