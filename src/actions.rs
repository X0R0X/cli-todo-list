extern crate xdg;

pub mod actions {
    use std::{fs, io};
    use std::io::{Write};
    use std::path::PathBuf;
    use crate::config::config::get_persistence_file_path;

    #[derive(Eq, PartialEq)]
    pub enum ActionResult {
        Ok,
        UnableToAddAction,
        UnableToRemoveAction,
    }

    fn load_file_to_vec(fp: &PathBuf) -> Vec<String> {
        fs::read_to_string(&fp)
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect()
    }

    fn save_vec_to_file(fp: &PathBuf, vec: &Vec<String>) -> io::Result<()> {
        fs::write(
            fp,
            format!("{}\n", vec.join("\n").as_str()).as_bytes(),
        )
    }

    pub fn list_actions() -> ActionResult {
        let fp = get_persistence_file_path();

        if fp.exists() {
            let f = fs::read_to_string(&fp).unwrap();
            for (i, line) in f.lines().enumerate() {
                println!("{}. {}", i + 1, line);
            }
        }
        ActionResult::Ok
    }

    pub fn add_action(action: String) -> ActionResult {
        let fp = get_persistence_file_path();

        if fp.exists() {
            let mut f = fs::OpenOptions::new()
                .append(true)
                .open(fp)
                .unwrap();

            if let Err(e) = writeln!(f, "{}", action) {
                println!("Unable to write to file: {}", e);
                ActionResult::UnableToAddAction
            } else {
                ActionResult::Ok
            }
        } else {
            fs::create_dir_all(&fp.parent().unwrap()).unwrap();
            if let Err(e) = fs::write(
                fp, format!("{}\n", action).as_bytes(),
            ) {
                println!(
                    "\nFailed to write the todo to a file: {}",
                    e.to_string()
                );
                ActionResult::UnableToAddAction
            } else {
                ActionResult::Ok
            }
        }
    }

    pub fn add_action_at_index(action: String, index: i64) -> ActionResult {
        let fp = get_persistence_file_path();
        if fp.exists() {
            let mut v = load_file_to_vec(&fp);

            let mut i: usize;
            if index < 0 {
                if (v.len() as i64 + index) < 0 {
                    i = 0;
                } else {
                    i = v.len() - (-index) as usize;
                }
            } else {
                i = index as usize;
                if i >= v.len() { i = v.len() - index as usize; }
            }

            v.insert(i, action);
            match save_vec_to_file(&fp, &v) {
                Ok(_) => ActionResult::Ok,
                Err(_) => {
                    ActionResult::UnableToAddAction
                }
            }
        } else {
            add_action(action)
        }
    }

    pub fn rm_action(action_n: i64) -> ActionResult {
        let fp = get_persistence_file_path();

        if fp.exists() {
            let mut v = load_file_to_vec(&fp);

            let index: usize;
            if action_n > -1 {
                index = action_n as usize - 1;
            } else if action_n < 0 {
                index = v.len() - (-action_n as usize);
            } else {
                return ActionResult::UnableToRemoveAction;
            }

            if index < v.len() {
                v.remove(index);
                match save_vec_to_file(&fp, &v) {
                    Ok(_) => ActionResult::Ok,
                    Err(e) => {
                        println!(
                            "Unable to remove {}. record: {}",
                            action_n,
                            e
                        );
                        ActionResult::UnableToRemoveAction
                    }
                }
            } else {
                ActionResult::UnableToRemoveAction
            }
        } else {
            ActionResult::UnableToRemoveAction
        }
    }
}