use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct CpArgs {
    pub src: String,
    pub des: String,
}

pub fn copy(arg: CpArgs) {
    let src = arg.src;
    let des = arg.des;
    let mut des_with_suffix = des.clone();

    let s = Path::new(&src);
    let d = Path::new(&des);

    if !s.is_file() && !s.is_dir() {
        eprintln!("wrong source!!!");
        return;
    }

    if s.is_file() {
        if d.is_dir() {
            #[cfg(unix)]
            {
                if !des.ends_with("/") {
                    des_with_suffix += "/";
                }
            }
            #[cfg(windows)]
            {
                if !des.ends_with("\\") {
                    des_with_suffix += "\\";
                }
            }
            let mut name_get: &str = "";
            if let Some(name) = s.file_name() {
                if let Some(name) = name.to_str() {
                    name_get = name;
                }
            }
            if name_get.is_empty() {
                eprintln!("can't get the file name!!!");
                return;
            }
            des_with_suffix += name_get;
        }
        if let Err(e) = fs::copy(src, des_with_suffix) {
            eprintln!("{e}");
        }
        return;
    }

    if !d.exists() {
        if let Err(e) = fs::create_dir_all(d) {
            eprintln!("{e}");
            return;
        }
        #[cfg(unix)]
        {
            if !des.ends_with("/") {
                des_with_suffix += "/";
            }
        }
        #[cfg(windows)]
        {
            if !des.ends_with("\\") {
                des_with_suffix += "\\";
            }
        }
    }
    copy_recursive_paths(CpArgs {
        src,
        des: des_with_suffix,
    });
}

fn copy_recursive_paths(arg: CpArgs) {
    let src = arg.src;
    let mut des = arg.des;

    let s = Path::new(&src);

    if s.is_file() {
        let mut name_get: &str = "";
        if let Some(name) = s.file_name()
            && let Some(name) = name.to_str()
        {
            name_get = name;
        }
        if name_get.is_empty() {
            eprintln!("can't get the file name!!!");
            return;
        }
        if let Err(e) = fs::copy(src.clone(), des + name_get) {
            eprintln!("{e}");
        }
        return;
    }

    if let Some(path) = s.file_name() {
        if let Some(path) = path.to_str() {
            #[cfg(unix)]
            {
                des = des + path + "/";
            }
            #[cfg(windows)]
            {
                des = des + path + "\\";
            }
            if let Err(e) = fs::create_dir(des.clone()) {
                eprintln!("{e}");
            }
            if let Ok(entries) = fs::read_dir(s) {
                for entry in entries.flatten() {
                    let src = entry.path();
                    if let Some(src) = src.to_str() {
                        copy_recursive_paths(CpArgs {
                            src: src.into(),
                            des: des.clone(),
                        });
                    }
                }
            }
        }
    }
}
