// extern crate lalrpop;

fn main() {
    lalrpop::process_root().expect("building parser");
}
