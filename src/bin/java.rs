use std::env;
use std::io;
use std::process::Command;

use quit;

use msgbox;

#[quit::main]
fn main() -> Result<(), io::Error> {
    let mut args : Vec<String> = env::args().collect();

    // Debug provided args
    match msgbox::create(
        "jw",
        &args.join(" "),
        msgbox::IconType::None
    ) {
        Ok(()) => {},
        Err(_) => quit::with_code(1)
    };

    let my_full_path_b = env::current_exe()?;
    let my_full_path = my_full_path_b.as_path();
    let exe_path = my_full_path.parent().unwrap();

    let mut java_exe = Command::new(exe_path.join("java_"));
    java_exe.args(args.drain(1..));

    quit::with_code(match java_exe.status() {
        Ok(status) => {
            let child_exit_code = status.code();
            match child_exit_code {
                Some(code) => code as u8 /* why */,
                None            => 1
            }
        },
        Err(_) => 1,
    });
}
