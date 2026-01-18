use super::Api;
use std::env;
use std::process;

pub fn exit(num: i32) {
    if num != 0 && num != 1 {
        eprintln!("need number 0 or 1 to exit!!!");
        return;
    }
    if let Ok(home) = env::var("HOME") {
        super::api(Api::WriteHistoryToFile((home + "/.drhistory").as_str()));
    }
    process::exit(num);
}
