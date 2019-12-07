use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn visit_dirs(dir: &Path, clz_nm: &str, cb: &dyn Fn(&DirEntry, &str)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, clz_nm, cb)?;
            } else {
                cb(&entry, clz_nm);
            }
        }
    }
    Ok(())
}

pub fn cb_eqn(de: &DirEntry, clz_nm: &str) {
    if let Some(p) = de.path().to_str() {
        println!("{}", p);
        if p.ends_with(clz_nm) {
            p
        } else {
            "no match path"
        }
    } else {
        "no match path"
        // Err((""))
    };
}

pub fn exp_dir(path: String, clz_nm: &str) {
    let clz_path = Path::new(path.as_str());
    println!(
        "the clz file path is {:?}",
        visit_dirs(clz_path, clz_nm, &cb_eqn)
    )
}

pub fn exp_cmp(path: String, clz_nm: &str) {}

pub fn exp_cop(path: String, clz_nm: &str) {}

pub fn exp_wlid(path: String, clz_nm: &str) {}

pub struct Bootpath {
    pub path: String,
}
pub struct Extpath {
    pub path: String,
}
pub struct Userpath {
    pub path: String,
}

impl Bootpath {
    pub fn readclz(&self, clz_nm: &str) {
        exp_wlid(self.path.clone(), clz_nm);
    }
}

impl Extpath {
    pub fn readclz(&self, clz_nm: &str) {
        exp_wlid(self.path.clone(), clz_nm);
    }
}

impl Userpath {
    pub fn readclz(&self, clz_nm: &str) {
        exp_dir(self.path.clone(), clz_nm);
    }
}
