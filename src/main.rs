use std::env;

fn main() {
    enum Cmd {
        Version,
        CP(String, String),
        HELP,
        None,
    }

    fn cmd_call_back(cmd: Cmd) -> u8 {
        match cmd {
            Cmd::Version => {
                println!("jvst 1.1.0");
                1
            }
            Cmd::HELP => {
                println!("
    USAGE:
        jvst [OPTIONS] [SUBCOMMAND]
    OPTIONS:
        $ jvst -V,--version Print version info and exit
        $ jvst -h,--help    Prints help information
		");
                1
            }
            _ => {
                println!("none");
                0
            }
        }
    }

    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }

    let code: &str = &args[1];
    let cmd = match code {
        "version" => {
            Cmd::Version
        }
        "help" => {
            Cmd::HELP
        }
        _ => {
            Cmd::None
        }
    };

    cmd_call_back(cmd);
}
