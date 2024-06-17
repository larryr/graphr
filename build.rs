extern crate lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/cgraph/node.lalrpop");
    let result = lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .always_use_colors()
        .emit_report(true)  // this is the default
        .log_verbose()
        .process_current_dir();
    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("lalrpop error: {:?}", e);
        }
    }
}