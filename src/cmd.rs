pub mod path;
use crate::cmd::path::Userpath;
pub enum Cmd {
    Version,
    CP(String, String),
    HELP,
    None,
}

pub fn cmd_call_back(cmd: Cmd) -> u8 {
    match cmd {
        Cmd::Version => {
            println!("jvst 1.1.0");
            1
        }
        Cmd::HELP => {
            println!(
                "
USAGE:
    jvst [OPTIONS] [SUBCOMMAND]
OPTIONS:
    $ jvst -V,--version Print version info and exit
    $ jvst -h,--help    Prints help information
    "
            );
            1
        }
        Cmd::CP(clzpath, clz) => {
            let user = Userpath { path: clzpath };
            user.readclz(&clz);
            // userpath = new userpath{path};
            // userpath.readclz(clz);
            1
        }
        _ => {
            println!("none");
            0
        }
    }
}
pub fn recv(args: &Vec<String>) {
    let code: &str = &args[1];
    let cmd = match code {
        "version" => Cmd::Version,
        "help" => Cmd::HELP,
        "cp" => Cmd::CP(args[2].clone(), args[3].clone()),
        _ => Cmd::None,
    };
    cmd_call_back(cmd);
}