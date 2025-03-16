// 引入classfile模块，用于处理类文件
use super::classfile;

// 引入标准库文件系统相关模块
use std::fs::{self, DirEntry, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

// 递归遍历目录的函数
// dir: 要遍历的目录路径
// clz_nm: 要查找的类名
// paths: 用于存储找到的路径
// cb: 回调函数，用于处理每个找到的文件入口
pub fn visit_dirs(
    dir: &Path,
    clz_nm: &str,
    paths: &mut Vec<String>,
    cb: &dyn Fn(&DirEntry, &mut Vec<String>, &str) -> io::Result<()>,
) -> io::Result<()> {
    // 检查传入的路径是否为目录
    if dir.is_dir() {
        // 读取目录中的所有条目
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // 如果是目录，则递归遍历
            if path.is_dir() {
                visit_dirs(&path, clz_nm, paths, cb)?;
            } else {
                // 如果是文件，调用回调函数处理
                cb(&entry, paths, clz_nm);
            }
        }
    }
    Ok(())
}

// 处理zip/jar文件的函数
// clz_zip: zip/jar文件路径
// paths: 用于存储找到的路径
// clz_nm: 要查找的类名
pub fn visit_zip(clz_zip: &Path, paths: &mut Vec<String>, clz_nm: &str) -> io::Result<()> {
    // 打开zip文件
    let file = File::open(clz_zip).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    // 遍历zip文件中的所有条目
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let name = file.name();
        // 如果文件名以指定的类名结尾
        if (name.ends_with(clz_nm)) {
            // 将路径添加到结果列表中
            paths.push(name.to_string());
            // 读取文件内容
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            // 使用classfile模块处理类文件
            classfile::readclz(&buffer);
            return Ok(());
        }
    }
    Ok(())
}

// 回调函数，用于检查文件名是否匹配类名
fn cb_eqn(de: &DirEntry, paths: &mut Vec<String>, clz_nm: &str) -> io::Result<()> {
    if let Some(p) = de.path().to_str() {
        // 如果文件路径以类名结尾
        if p.ends_with(clz_nm) {
            // 将路径添加到结果列表中
            paths.push(p.to_string());
            // 打开并读取文件内容
            let mut file = File::open(p)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            // 使用classfile模块处理类文件
            classfile::readclz(&buffer);
        } else {
            return Ok(());
        }
    }
    Ok(())
}

// 导出目录处理函数
// path: 目录路径
// paths: 用于存储找到的路径
// clz_nm: 要查找的类名
pub fn exp_dir(path: String, paths: &mut Vec<String>, clz_nm: &str) {
    let clz_path = Path::new(path.as_str());
    println!(
        "the clz file path is {:?}",
        visit_dirs(clz_path, clz_nm, paths, &cb_eqn)
    );
}

// 导出压缩文件处理函数（尚未实现）
pub fn exp_cmp(path: String, clz_nm: &str) {}

// 导出复制文件处理函数，用于处理zip/jar文件
pub fn exp_cop(path: String, paths: &mut Vec<String>, clz_nm: &str) {
    let clz_path = Path::new(path.as_str());
    visit_zip(clz_path, paths, clz_nm);
}

// 导出通配符处理函数（尚未实现）
pub fn exp_wlid(path: String, clz_nm: &str) {}
