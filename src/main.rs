use contacts::command::Command;
use std::env;
use std::process;
fn main() {
    let command = Command::parse(env::args());
    let cmd = match command {
        Ok(cmd) => cmd,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };
    println!("{:?}", cmd);
}
