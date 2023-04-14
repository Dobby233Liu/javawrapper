use std::env;
use std::process;

use quit;
use confy;

use msgbox;

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct WrapperConfig {
    orig_exe: String,
}
impl ::std::default::Default for WrapperConfig {
    fn default() -> Self { Self { orig_exe: "java_".into() } }
}

#[quit::main]
fn main() -> Result<(), ::std::io::Error> {
    let cfg : WrapperConfig = confy::load("javawrapper")?;

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
    let exe_path = my_full_path.parent()?;

    let mut java_exe = process::Command::new(
        exe_path.join(cfg.orig_exe)
    );
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
