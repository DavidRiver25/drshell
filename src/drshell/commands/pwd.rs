use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(err) => {
            eprintln!(
                "{}",
                "can't get the current work directory: ".to_string() + &err.to_string()
            );
        }
    }
}
