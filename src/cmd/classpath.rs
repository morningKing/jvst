use super::visit;

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
        visit::exp_wlid(self.path.clone(), clz_nm);
    }
}

impl Extpath {
    pub fn readclz(&self, clz_nm: &str) {
        visit::exp_wlid(self.path.clone(), clz_nm);
    }
}

impl Userpath {
    pub fn readclz(&self, clz_nm: &str) {
        let mut paths = self.paths.clone();
        if self.path.ends_with(".zip") {
            visit::exp_cop(self.path.clone(), &mut paths, clz_nm);
            for path in paths {
                println!("abs path :{}", path);
            }
        } else if self.path.ends_with(".jar") {
            visit::exp_cop(self.path.clone(), &mut paths, clz_nm);
            for path in paths {
                println!("abs path :{}", path);
            }
        } else {
            visit::exp_dir(self.path.clone(), &mut paths, clz_nm);
            for path in paths {
                println!("abs path :{}", path);
            }
        }
    }
}
