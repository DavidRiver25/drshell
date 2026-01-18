use lazy_static::lazy_static;
use std::env::{self, current_dir};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exe {
    name: String,
    path: String,
}

impl Clone for Exe {
    fn clone(&self) -> Self {
        Exe {
            name: self.name.clone(),
            path: self.path.clone(),
        }
    }
}

impl Exe {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[allow(dead_code)]
    fn get_path(&self) -> String {
        self.path.clone()
    }
}

// pub fn if_executable(file: &str) -> Option<String> {
//     for exe in find_exes() {
//         if exe.get_name() == file {
//             return Some(exe.get_path());
//         }
//     }
//
//     return None;
// }

pub fn if_executable(file: &str) -> Option<PathBuf> {
    let Ok(path_var) = env::var("PATH") else {
        return None;
    };
    for dir in env::split_paths(&path_var) {
        let full_path = dir.join(format!("{}{}", file, env::consts::EXE_SUFFIX));
        if if_exe(&full_path).is_some() {
            return Some(full_path);
        }
    }
    None
}

/* ??? duplicate ??? */
pub fn find_exes() -> Vec<Exe> {
    let mut exes = Vec::new();
    let Ok(path_var) = env::var("PATH") else {
        return exes;
    };
    for dir in env::split_paths(&path_var) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(exe) = if_exe(&path) {
                        exes.push(exe);
                    }
                }
            }
        }
    }

    exes
}

pub fn find_current_dir_files() -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(dir) = env::current_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name();
                    if let Some(name) = name.to_str() {
                        files.push(name.to_string());
                    }
                }
            }
        }
    }

    files
}

fn if_exe(path: &PathBuf) -> Option<Exe> {
    if path.is_file() {
        let Some(p) = path.to_str() else {
            return None;
        };
        #[cfg(unix)]
        {
            let Ok(metadata) = fs::metadata(&path) else {
                return None;
            };
            let mode = metadata.permissions().mode();
            if mode & 0o111 != 0 {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        return Some(Exe {
                            name: name.to_string(),
                            path: p.to_string(),
                        });
                    }
                }
            }
        }
        #[cfg(windows)]
        {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    if name.ends_with(".exe") {
                        let name = name
                            .chars()
                            .take(name.chars().count().saturating_sub(4))
                            .collect::<String>();
                        return Some(Exe {
                            name,
                            path: p.to_string(),
                        });
                    }
                }
            }
        }
    }

    None
}
