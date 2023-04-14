// Adds version information to our Windows executable.

extern crate embed_resource;

fn main() {
    println!("cargo:rerun-if-changed=platform/windows/java.rc");
    embed_resource::compile("platform/windows/java.rc", embed_resource::NONE);
}