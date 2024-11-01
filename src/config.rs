const DATA_DIR_NAME: &str = "todo-list";

const PERSISTENCE_FILE_NAME: &str = "todo.txt";

pub mod config {
    use std::path::{Path, PathBuf};
    use xdg::BaseDirectories;
    use crate::config::{DATA_DIR_NAME, PERSISTENCE_FILE_NAME};

    pub fn get_persistence_file_path() -> PathBuf {
        let bd = BaseDirectories::new();
        match bd {
            Ok(result) => {
                Path::new(
                    format!(
                        "{}{}/{}",
                        result.get_data_home().to_str().unwrap(),
                        DATA_DIR_NAME,
                        PERSISTENCE_FILE_NAME
                    ).as_str()
                ).to_path_buf()
            }
            Err(_) => {
                Path::new(
                    format!(
                        "~/{}/{}",
                        DATA_DIR_NAME,
                        PERSISTENCE_FILE_NAME
                    ).as_str()
                ).to_path_buf()
            }
        }
    }
}