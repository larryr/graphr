use lalrpop_util::lalrpop_mod;
use graphr::cgraph::grammar;


#[test]
fn dot1() {

    println!("dot lang test 1");
    assert!(grammar::TermParser::new().parse("2").is_ok())
}

#[test]
fn dot2() {

    println!("dot lang test 2");
    assert!(grammar::TermParser::new().parse("2.3").is_ok())
}

#[test]
fn dot3() {

    println!("dot lang test 3");
    assert!(grammar::TermParser::new().parse("2.3e-4").is_ok())
}

#[test]
fn dot4() {

    println!("dot lang test 4");
    assert!(grammar::TermParser::new().parse("2.3e+4").is_ok())
}

#[test]
fn dot5() {

    println!("dot lang test 5");
    assert!(grammar::TermParser::new().parse("2.3e4").is_ok())
}

#[test]
fn dot6() {

    println!("dot lang test 6");
    assert!(grammar::TermParser::new().parse("2.3e4").is_ok())
}

#[test]
fn dot7() {

    println!("dot lang test 7");
    assert!(grammar::TermParser::new().parse("2.3e+4").is_ok())
}

#[test]
fn dot8() {

    println!("dot lang test 8");
    assert!(grammar::TermParser::new().parse("2.3e-4").is_ok())
}

#[test]
fn dot9() {

    println!("dot lang test 9");
    assert!(grammar::TermParser::new().parse("2.3e4").is_ok())
}

#[test]
fn dot10() {

    println!("dot lang test 10");
    assert!(grammar::TermParser::new().parse("2.3e-4").is_ok())
}

#[test]
fn dot11() {

    println!("dot lang test 11");
    assert!(grammar::TermParser::new().parse("2.3e4").is_ok())
}

#[test]
fn dot12() {

    println!("dot lang test 12");
    assert!(grammar::TermParser::new().parse("2.3e-4").is_ok())
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
