use std::env;
use std::process;
/*use std::fs::File;*/

use quit;
use confy;

/*use msgbox;*/

#[derive(::serde::Serialize, ::serde::Deserialize)]
struct WrapperConfig {
    orig_exe: String,
}
impl ::std::default::Default for WrapperConfig {
    fn default() -> Self { Self { orig_exe: "java_".into() } }
}

fn prevent_obsolete_params(v : &String) -> bool {
    // Very rough
    v != "-Xfuture" && v != "-Xincgc"
}

#[quit::main]
fn main() -> Result<(), ::std::io::Error> {
    let cfg : WrapperConfig = confy::load("javawrapper")?;

    let mut our_args : Vec<String> = env::args().collect();

    // Debug provided args
    /*
    match msgbox::create(
        "jw",
        &our_args.join(" "),
        msgbox::IconType::None
    ) {
        Ok(()) => {},
        Err(_) => quit::with_code(1)
    };
    */

    let my_full_path_b = env::current_exe()?;
    let my_full_path = my_full_path_b.as_path();
    let exe_path = my_full_path.parent().unwrap();

    // Debug out
    /*
    let out_file = File::create("java.log")?;
    let out_file_e = out_file.try_clone()?;
    */

    let mut java_exe = process::Command::new(
        exe_path.join(cfg.orig_exe)
    );
    java_exe.args(our_args.drain(1..).filter(prevent_obsolete_params))
    /*
            .stdout(process::Stdio::from(out_file))
            .stderr(process::Stdio::from(out_file_e))
    */
    ;

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
