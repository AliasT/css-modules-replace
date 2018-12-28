extern crate regex;
use std::fs;
use regex::Regex;
use std::path::Path;
use std::io;

fn main() {
    let _ = visit_dirs(Path::new("/Users/chai/Documents/work/scratch-gui/src"), &|f: &fs::DirEntry| {
        replace(&f.path());
    });
}


fn visit_dirs(dir: &Path, cb: &Fn(&fs::DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn replace(path: &Path) {
    if let Ok(content) = fs::read_to_string(&path) {
        let re = Regex::new(r"(styles?)\[(?P<m>[^]]+)\]").unwrap(); // sytle['foo-bar-zoo'] --> style.fooBarZoo
        let ire = Regex::new(r"-([a-z])").unwrap();

        let after = re.replace_all(&content, |caps: &regex::Captures| {
            let inner = ire.replace_all(&caps[2], |caps: &regex::Captures| {
                caps[1].to_uppercase()
            }); // 中括号内的内容
            println!("-------{}-------", inner);
            format!("{}.{}", &caps[1], inner.replace("'", ""))
        });

        let _ = fs::write(path, after.as_ref());
    }
}
