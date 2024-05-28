extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .always_use_colors()
        .process_current_dir()
        .unwrap();
}