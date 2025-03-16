// 引入标准库环境模块，用于获取命令行参数
use std::env;
// 导入本地cmd模块
mod cmd;

// 程序入口函数
fn main() {
    // 创建一个动态数组用于存储命令行参数
    let mut args = Vec::new();
    // 遍历环境变量中的所有参数并添加到args数组中
    for arg in env::args() {
        args.push(arg);
    }
    // 调用cmd模块中的recv函数处理命令行参数
    crate::cmd::recv(&args);
}
