use super::parse;
use std::fs::{self, DirEntry, File};
use std::io;
use std::path::Path;

pub fn visit_dirs(
    dir: &Path,
    clz_nm: &str,
    paths: &mut Vec<String>,
    cb: &dyn Fn(&DirEntry, &mut Vec<String>, &str),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, clz_nm, paths, cb)?;
            } else {
                cb(&entry, paths, clz_nm);
            }
        }
    }
    Ok(())
}

pub fn visit_zip(clz_zip: &Path, paths: &mut Vec<String>, clz_nm: &str) -> io::Result<()> {
    let file = File::open(clz_zip).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let name = file.name();
        if (name.ends_with(clz_nm)) {
            paths.push(name.to_string());
            return Ok(());
        }
    }
    Ok(())
}

pub fn cb_eqn(de: &DirEntry, paths: &mut Vec<String>, clz_nm: &str) {
    if let Some(p) = de.path().to_str() {
        if p.ends_with(clz_nm) {
            paths.push(p.to_string());
            p
        } else {
            "no match path"
        }
    } else {
        "no match path"
    };
}

pub fn exp_dir(path: String, paths: &mut Vec<String>, clz_nm: &str) {
    let clz_path = Path::new(path.as_str());
    println!(
        "the clz file path is {:?}",
        visit_dirs(clz_path, clz_nm, paths, &cb_eqn)
    );
}

pub fn exp_cmp(path: String, clz_nm: &str) {}

pub fn exp_cop(path: String, paths: &mut Vec<String>, clz_nm: &str) {
    let clz_path = Path::new(path.as_str());
    visit_zip(clz_path, paths, clz_nm);
}

pub fn exp_wlid(path: String, clz_nm: &str) {}

pub struct Bootpath {
    pub path: String,
    pub paths: Vec<String>,
}
pub struct Extpath {
    pub path: String,
    pub paths: Vec<String>,
}
pub struct Userpath {
    pub path: String,
    pub paths: Vec<String>,
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
        let mut paths = self.paths.clone();
        if (self.path.ends_with(".zip") || self.path.ends_with(".jar")) {
            exp_cop(self.path.clone(), &mut paths, clz_nm);
            for path in paths {
                println!("{}:", path);
                parse::show_clz_bytes(&path);
            }
        } else {
            exp_dir(self.path.clone(), &mut paths, clz_nm);
            for path in paths {
                println!("{}:", path);
                parse::show_clz_bytes(&path);
            }
        }
    }
}
