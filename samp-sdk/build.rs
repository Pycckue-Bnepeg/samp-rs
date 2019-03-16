use colored::*;

fn main() {
    if cfg!(not(target_arch = "x86")) {
        eprintln!("{}: samp-sdk can be compiled only for {} target arch.", "error".red(), "x86".green());
        eprintln!("{}: install a i686 toolchain for example {}", "help".yellow(), "rustup install stable-i686-pc-windows-msvc".green());
        panic!();
    }
}
