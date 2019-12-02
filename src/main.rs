use std::borrow::Borrow;

fn main() {
    enum Cmd {
        Version,
    }
    impl Cmd {
        fn call(&self) {
//            if self == Cmd::Version {
//                println!("jvst++")
//            }
        }
    }
    fn cmd_call_back(cmd: Cmd) -> u8 {
        match cmd {
            Cmd::Version => {
                println!("jvst++!");
                1
            }
            _ => {
                println!("unknown cmd");
                0
            }
        }
    }
    let version = Cmd::Version;
    let i = cmd_call_back(version);
}
