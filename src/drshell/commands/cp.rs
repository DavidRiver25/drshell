use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct CpArgs {
    pub src: String,
    pub des: String,
}

pub fn copy(arg: CpArgs) {
    let src = arg.src;
    let mut des = arg.des;

    let s = Path::new(&src);
    let d = Path::new(&des);

    if !s.is_file() && !s.is_dir() {
        eprintln!("wrong source!!!");
        return;
    }

    if !d.exists() {
        if let Err(e) = fs::create_dir_all(d) {
            eprintln!("{e}");
            return;
        }
    }

    #[cfg(unix)]
    {
        if !des.ends_with("/") {
            des += "/";
        }
    }
    #[cfg(windows)]
    {
        if !des.ends_with("\\") {
            des += "\\";
        }
    }

    if s.is_file() {
        if let Some(name) = s.file_name() {
            if let Some(name) = name.to_str() {
                if let Err(e) = fs::copy(src.clone(), des + name) {
                    eprintln!("{e}");
                }
            }
        }
        return;
    }

    if let Some(path) = s.file_name() {
        if let Some(path) = path.to_str() {
            des += path;
            if let Err(e) = fs::create_dir(des.clone()) {
                eprintln!("{e}");
            }
            if let Ok(entries) = fs::read_dir(s) {
                for entry in entries.flatten() {
                    let src = entry.path();
                    if let Some(src) = src.to_str() {
                        copy(CpArgs {
                            src: src.into(),
                            des: des.clone(),
                        });
                    }
                }
            }
        }
    }
}
