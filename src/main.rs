use contacts::cli;
use contacts::command::Command;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // 诊断信息走 stderr。用 println! 的话,
            // `contacts list > out.txt` 会把错误混进数据里。
            eprintln!("错误: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::parse(env::args().skip(1))?;
    cli::run(cmd)?;
    Ok(())
}
