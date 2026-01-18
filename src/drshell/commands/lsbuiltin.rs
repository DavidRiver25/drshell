use super::BUILTIN_CMDS;

pub fn lsbuiltin() {
    for cmd in BUILTIN_CMDS.iter() {
        println!("{cmd}");
    }
}
