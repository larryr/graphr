use lalrpop_util::lalrpop_mod;
use graphr::cgraph::grammar;


#[test]
fn dot1() {

    println!("dot lang test 1");
    assert!(grammar::TermParser::new().parse("2").is_ok())
}