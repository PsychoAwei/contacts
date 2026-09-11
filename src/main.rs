use contacts::command::Command;
use std::env;
use std::process;
const CONTACTS_PATH: &str = "./contacts.json";
fn main() {
    let command = Command::parse(env::args());
    let cmd = match command {
        Ok(cmd) => cmd,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };
    // if let Err(err) = app::run(cmd) {
    //     println!("{err}");
    //     process::exit(1);
    // }
}
