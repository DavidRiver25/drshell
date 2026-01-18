use std::env;
use std::sync::Mutex;

pub fn cd(mut dir: String) {
    static LAST_DIR: Mutex<String> = Mutex::new(String::new());

    match dir.as_str() {
        "" => {
            eprintln!("{}", "cd: ".to_string() + ": No such file or directory");
            return;
        }
        "-" => {
            dir = LAST_DIR.lock().unwrap().clone();
            if dir.is_empty() {
                eprintln!("no previous worked directory");
                return;
            }
        }
        "~" => {
            dir = match env::var("HOME") {
                Ok(home) => home,
                Err(err) => {
                    eprintln!(
                        "{}",
                        "can't get the directory of home: ".to_string() + &err.to_string()
                    );
                    return;
                }
            }
        }
        &_ => {
            let mut chars = dir.chars();
            if chars.next().unwrap_or_default() == '~' && chars.next().unwrap_or_default() == '/' {
                let home = match env::var("HOME") {
                    Ok(home) => home,
                    Err(err) => {
                        eprintln!(
                            "{}",
                            "can't get the directory of home: ".to_string() + &err.to_string()
                        );
                        return;
                    }
                };
                let subdir: String = dir.chars().skip(1).collect();
                dir = home + &subdir;
            };
        }
    }
    let last_dir = match env::current_dir() {
        Ok(path) => match path.into_os_string().into_string() {
            Ok(path) => path,
            Err(_osstring) => {
                eprintln!("can't resolve the current working directory");
                return;
            }
        },
        Err(err) => {
            eprintln!(
                "{}",
                "can't resolve the current working directory: ".to_string() + &err.to_string()
            );
            return;
        }
    };
    match env::set_current_dir(&dir) {
        Err(_err) => {
            eprintln!(
                "{}",
                "cd: ".to_string() + &dir + ": No such file or directory"
            );
        }
        Ok(_ok) => {
            *LAST_DIR.lock().unwrap() = last_dir;
        }
    }
}
