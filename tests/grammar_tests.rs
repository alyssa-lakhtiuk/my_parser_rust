use pest::Parser;
use anyhow::anyhow;
use simple_yaml_parser_kma::*;

#[test]
fn basic_test() -> anyhow::Result< () > {
    let correct = "---\nsome-content: 15\n...";
    let got = Grammar::parse(Rule::full_yaml, correct)?;
    println!("{:?}", got);

    Ok(())
}


#[test]
fn full_yaml_rule_test() -> anyhow::Result< () > {
    let correct = "a: 1\nb: 2\n";
    let pair = Grammar::parse(Rule::full_yaml, "---\na: 1\nb: 2\n...")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 4 );
    assert_eq!( pair.as_span().end(), 14 );

    let pair = Grammar::parse(Rule::full_yaml, "");
    assert!(pair.is_err());


    let correct = "12: AB\n34: cd\n56: \n  - a: 1\n  - b: 2\n  - c: 3\na1: 12\n";
    let file_path = String::from("./test_files/test_file2.txt");
    let content_from_file = read_file(file_path.as_str());

    let pair = Grammar::parse(Rule::full_yaml, content_from_file.as_str())?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct );

    Ok(())
}

#[test]
fn object_rule_test() -> anyhow::Result< () > { 
    let correct = "a: 1\nb: 2\n";
    let pair = Grammar::parse(Rule::object, "a: 1\nb: 2\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );
    Ok(())
}

#[test]
fn helpnode_rule_test() -> anyhow::Result< () > { 
    let correct = "a: 1\n";
    let pair = Grammar::parse(Rule::helpnode, "a: 1\n11111")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 5 );

    let pair = Grammar::parse(Rule::helpnode, "");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::helpnode, "{anything here}");
    assert!(pair.is_err());
    Ok(())
}

#[test]
fn subnode_rule_test() -> anyhow::Result< () > { 
    let correct = "  a: 1\n  b: 2";
    let pair = Grammar::parse(Rule::subnode, "  a: 1\n  b: 2")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 13 );

    let pair = Grammar::parse(Rule::subnode, "");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::subnode, "key: value");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn key_value_rule_test() -> anyhow::Result< () > { 
    Ok(())
}

#[test]
fn value_rule_test() -> anyhow::Result< () > { 
    Ok(())
}

#[test]
fn scalar_rule_test() -> anyhow::Result< () > { 
    Ok(())
}

#[test]
fn number_rule_test() -> anyhow::Result< () > { 
    Ok(())
}

#[test]
fn boolean_rule_test() -> anyhow::Result< () > { 
    Ok(())
}

#[test]
fn null_rule_test() -> anyhow::Result< () > { 
    Ok(())
}


#[test]
fn string_rule_test() -> anyhow::Result< () > { 
    Ok(())
}