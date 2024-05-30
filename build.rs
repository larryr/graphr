extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .always_use_colors()
        .emit_report(true)  // this is the default
        .log_verbose()
        .process_current_dir()
        .unwrap();
}