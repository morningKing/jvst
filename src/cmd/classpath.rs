// 引入visit模块，用于访问文件系统
use super::visit;

// 定义引导路径结构体，用于处理Java引导类路径
pub struct Bootpath {
    pub path: String,      // 路径字符串
    pub paths: Vec<String>, // 路径数组
}

// 定义扩展路径结构体，用于处理Java扩展类路径
pub struct Extpath {
    pub path: String,      // 路径字符串
    pub paths: Vec<String>, // 路径数组
}

// 定义用户路径结构体，用于处理用户自定义类路径
pub struct Userpath {
    pub path: String,      // 路径字符串
    pub paths: Vec<String>, // 路径数组
}

// 为Bootpath实现方法
impl Bootpath {
    // 读取类文件的方法，传入类名，调用visit模块的exp_wlid函数
    pub fn readclz(&self, clz_nm: &str) {
        visit::exp_wlid(self.path.clone(), clz_nm);
    }
}

// 为Extpath实现方法
impl Extpath {
    // 读取类文件的方法，传入类名，调用visit模块的exp_wlid函数
    pub fn readclz(&self, clz_nm: &str) {
        visit::exp_wlid(self.path.clone(), clz_nm);
    }
}

// 为Userpath实现方法
impl Userpath {
    // 读取类文件的方法，根据路径类型调用不同的函数处理
    pub fn readclz(&self, clz_nm: &str) {
        let mut paths = self.paths.clone();
        // 如果路径以.zip结尾，使用exp_cop函数处理压缩文件
        if self.path.ends_with(".zip") {
            visit::exp_cop(self.path.clone(), &mut paths, clz_nm);
            // 打印所有找到的绝对路径
            for path in paths {
                println!("abs path :{}", path);
            }
        // 如果路径以.jar结尾，也使用exp_cop函数处理JAR文件
        } else if self.path.ends_with(".jar") {
            visit::exp_cop(self.path.clone(), &mut paths, clz_nm);
            // 打印所有找到的绝对路径
            for path in paths {
                println!("abs path :{}", path);
            }
        // 否则，视为目录，使用exp_dir函数遍历处理
        } else {
            visit::exp_dir(self.path.clone(), &mut paths, clz_nm);
            // 打印所有找到的绝对路径
            for path in paths {
                println!("abs path :{}", path);
            }
        }
    }
}
