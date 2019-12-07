use std::env;
mod cmd;
fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }
    crate::cmd::recv(&args);
}
