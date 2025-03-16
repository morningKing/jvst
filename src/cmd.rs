// 声明多个子模块
pub mod classfile;    // 类文件处理模块
pub mod classpath;    // 类路径处理模块
pub mod visit;        // 文件访问模块

// 引入项目中定义的Userpath结构
use crate::cmd::classpath::Userpath;

// 定义命令枚举类型，表示程序支持的命令
pub enum Cmd {
    Version,          // 版本信息命令
    CP(String, String), // 类路径命令，携带两个字符串参数
    HELP,             // 帮助信息命令
    None,             // 无命令/未知命令
}

// 命令处理回调函数，根据不同的命令执行相应的操作
pub fn cmd_call_back(cmd: Cmd) -> u8 {
    match cmd {
        // 处理版本命令：显示版本信息并返回1
        Cmd::Version => {
            println!("jvst 1.1.0");
            1
        }
        // 处理帮助命令：显示帮助信息并返回1
        Cmd::HELP => {
            println!(
                "
USAGE:
    jvst [OPTIONS] [SUBCOMMAND]
OPTIONS:
    $ jvst -V,--version    Print version info and exit
    $ jvst -h,--help       Prints help information
    $ jvst -cp,--classpath show class abs path
    "
            );
            1
        }
        // 处理类路径命令：创建用户路径对象并调用readclz方法，返回1
        Cmd::CP(clzpath, clz) => {
            let paths: Vec<String> = Vec::new();
            let user = Userpath {
                path: clzpath,
                paths: paths,
            };
            user.readclz(&clz);
            1
        }
        // 处理未知命令：显示"none"并返回0
        _ => {
            println!("none");
            0
        }
    }
}

// 接收并解析命令行参数，将其转换为Cmd枚举并调用处理函数
pub fn recv(args: &Vec<String>) {
    let code: &str = &args[1];
    // 根据输入参数匹配对应的命令
    let cmd = match code {
        "version" => Cmd::Version,
        "help" => Cmd::HELP,
        "cp" => Cmd::CP(args[2].clone(), args[3].clone()),
        _ => Cmd::None,
    };
    // 调用命令处理回调函数
    cmd_call_back(cmd);
}
