use lalrpop_util::lalrpop_mod;
use graphr::cgraph::grammar;


#[test]
fn dot01() {

    println!("dot lang test 1");
    assert!(grammar::TermParser::new().parse("2").is_ok())
}

#[test]
fn dot01a() {

    println!("dot lang test 1");
    assert!(grammar::TermParser::new().parse("22").is_ok())
}
#[test]
fn dot01b() {

    println!("dot lang test 1");
    assert!(grammar::TermParser::new().parse("-2").is_ok())
}
#[test]
fn dot01c() {

    println!("dot lang test 1");
    assert!(grammar::TermParser::new().parse("(((2)))").is_ok())
}

#[test]
fn dot02() {

    println!("dot lang test 2");
    assert!(grammar::TermParser::new().parse("2.3").is_ok())
}

#[test]
fn dot03() {

    println!("dot lang test 3");
    assert!(grammar::TermParser::new().parse("-2.3").is_ok())
}

#[test]
fn dot04() {

    println!("dot lang test 4");
    assert!(grammar::TermParser::new().parse("2.3e+4").is_ok())
}

#[test]
fn dot05() {

    println!("dot lang test 5");
    assert!(grammar::TermParser::new().parse("2.3-e4").is_ok())
}

#[test]
fn dot06() {

    println!("dot lang test 6");
    assert!(grammar::TermParser::new().parse("2.3e4").is_ok())
}

#[test]
fn dot07() {

    println!("dot lang test 7");
    assert!(grammar::TermParser::new().parse("-2.3e4").is_ok())
}

#[test]
fn dot08() {

    println!("dot lang test 8");
    assert!(grammar::TermParser::new().parse("-2.3e-4").is_ok())
}

#[test]
fn dot09() {

    println!("dot lang test 9");
    assert!(grammar::TermParser::new().parse("+2.3e4").is_ok())
}

#[test]
fn dot10() {

    println!("dot lang test 10");
    assert!(grammar::TermParser::new().parse("0100").is_ok())
}

#[test]
fn dot11() {

    println!("dot lang test 11");
    assert!(grammar::TermParser::new().parse("0x800").is_ok())
}

#[test]
fn dot12() {

    println!("dot lang test 12");
    assert!(grammar::TermParser::new().parse("100").is_ok())
}

#[test]
fn dot13() {

    println!("dot lang test 13");
    assert!(grammar::TermParser::new().parse("2.3e4").is_ok())
}

#[test]
fn dot14() {

    println!("dot lang test 14");
    assert!(grammar::TermParser::new().parse("2.3e-4").is_ok())
}


#[test]
fn dot15() {

    println!("dot lang test 15");
    assert!(grammar::GraphParser::new().parse(r#"digraph G {  a -> b  label="this is a graph" }"#).is_ok())
}