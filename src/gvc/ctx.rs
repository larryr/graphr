
use std::env;

pub struct Ctx {

}

pub fn new() -> Ctx {
    println!("new Ctx");
    Ctx {

    }
}
/** notes
- keep in context any errors over all operations that can be emitted on exit
-
**/
impl Ctx {
    pub fn print(&self) {
        println!("Hello, from module ctx!");
    }

    pub fn parse_args(&self) {
        println!("Hello, from module ctx2!");
        for arg in env::args().skip(1) {
            println!("{}", arg);
        }
    }

    pub fn next_input_graph(&self) -> Vec<&str> {
        let mut v = vec!["one", "two", "three"];
        v.push("four");
        v
    }

    /**
    * layout_jobs - layout graph according to options in context.
    * # Arguments
    * `g` - graph structure
    * # Remarks
    */
    pub fn layout_jobs(&self, g: &str) {
        println!("layout_jobs: {}", g);
    }

    //
    // Render functions
    // not implemented:
    // render()
    //

    /**
    * render_jobs - render layout according to options in context.
    * # Arguments
    * `g` - graph structure
    * # Remarks
    */
    pub fn render_jobs(&self, g: &str) {
        println!("render_jobs: {}", g);
    }

    pub fn finalize(&self) {
        println!("finalize");
    }

    pub fn plugin_list(&self, kind : String, cnt: i32) {
        println!("plugin_list");
    }
}