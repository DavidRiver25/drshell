use super::super::env as drshell_env;
use super::BUILTIN_CMDS;

pub fn r#type(command: String) {
    for cmd in BUILTIN_CMDS.iter() {
        if &command.as_str() == cmd {
            println!("{}", command + " is a shell builtin");
            return;
        }
    }
    if command.is_empty() {
        eprintln!("{}", command + ": not found");
        return;
    }
    if let Some(path) = drshell_env::if_executable(&command) {
        println!("{}", command + " is " + &path.display().to_string());
    } else {
        eprintln!("{}", command + ": not found");
    }
}
