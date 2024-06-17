use lalrpop_util::lalrpop_mod;
use graphr::cgraph::grammar;


#[test]
fn dot01() {

    println!("dot lang test 1");
    assert!(grammar::GraphParser::new().parse(r#"graph G { }"#).is_ok())
}

#[test]
fn dot01a() {

    println!("dot lang test 1a");
    assert!(grammar::GraphParser::new().parse(r#"graph { }"#).is_ok())
}

#[test]
fn dot02() {
    println!("dot lang test 2");
    match grammar::GraphParser::new().parse(r#"graph G { a = b }"#) {
        Ok(_) => println!("Parse 2 successful"),
        Err(e) => {
            println!("Parse failed with error: {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn dot03() {
    println!("dot lang test 3");
    match grammar::GraphParser::new().parse(r#"graph G { a = "a string" }"#) {
        Ok(_) => println!("Parse 3 successful"),
        Err(e) => {
            println!("Parse 3 failed with error: {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn dot14() {

    println!("dot lang test 14");
    match grammar::GraphParser::new().parse(r#"digraph G {  a -> b  ;}"#) {
        Ok(_) => println!("Parse 14 successful"),
        Err(e) => {
            println!("Parse 14 failed with error: {:?}", e);
            assert!(false);
        }
    }}



#[test]
fn dot15() {

    println!("dot lang test 15");
    match grammar::GraphParser::new().parse(r#"digraph G {  a -> b  label=x}"#) {
        Ok(_) => println!("Parse 15 successful"),
        Err(e) => {
            println!("Parse 15 failed with error: {:?}", e);
            assert!(false);
        }
    }
}


// the following test will fail if "compass_pt" is defined to only allow the
// strings "n", "ne", "e", "se", "s", "sw", "w", "nw".  Where if it is to allow
// any "ident" (and i use id_ident) it will pass. The dot grammar explicity states
// the compass_pt values are NOT keywords.
#[test]
fn dot15a() {

    println!("dot lang test 15");
    match grammar::GraphParser::new().parse(r#"digraph G {  a -> b ;  n -> d}"#) {
        Ok(_) => println!("Parse 15a successful"),
        Err(e) => {
            println!("Parse 15a failed with error: {:?}", e);
            assert!(false);
        }
    }
}


#[test]
fn dot15b() {

    println!("dot lang test 15");
    match grammar::GraphParser::new().parse(r#"digraph G {  a -> b ;  label = 5}"#) {
        Ok(_) => println!("Parse 15b successful"),
        Err(e) => {
            println!("Parse 15b failed with error: {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn dot16() {

    println!("dot lang test 16");
    match grammar::GraphParser::new().parse(r#"
    graph G2 {
        a -- b ;  label = 5
    }
    "#) {
        Ok(_) => println!("Parse 16 successful"),
        Err(e) => {
            println!("Parse 16 failed with error: {:?}", e);
            assert!(false);
        }
    }
}


#[test]
fn dot17() {

    println!("dot lang test 17");
    match grammar::GraphParser::new().parse(r#"
    digraph G {
        a -> {b c} ;  label = 5
    }
    "#) {
        Ok(_) => println!("Parse 17 successful"),
        Err(e) => {
            println!("Parse 17 failed with error: {:?}", e);
            assert!(false);
        }
    }
}


#[test]
fn dot200() {

    println!("dot lang test 200");
    match grammar::Expr1Parser::new().parse(r#"x:ne"#) {
        Ok(_) => println!("Parse 200 successful"),
        Err(e) => {
            println!("Parse 200 failed with error: {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn dot210() {

    println!("dot lang test 210");
    match grammar::Expr2Parser::new().parse(r#"x=y  ;  a=b ; d=f, "str"=5"#) {
        Ok(_) => println!("Parse 210 successful"),
        Err(e) => {
            println!("Parse 210 failed with error: {:?}", e);
            assert!(false);
        }
    }
}

