use pest::Parser;
use anyhow::anyhow;
pub use simple_yaml_parser_kma::*;

#[test]
fn serialization_test() {
    assert_eq!(parse_yaml_file("---\na: b\n...").unwrap(), YAMLValue::Object(vec![("a", YAMLValue::String("b"))]));
    assert_eq!(
        parse_yaml_file("---\nkey: value\nnum: 100\nnull: null\nbool: false\narr: \n  a1: 11.0\n  b1: 12.0\n...").unwrap(),
        YAMLValue::Object(vec![
            ("key", YAMLValue::String("value")),
            ("num", YAMLValue::Number(100.0)),
            ("null", YAMLValue::Null),
            ("bool", YAMLValue::Boolean(false)),
            ("arr", YAMLValue::Array(vec![YAMLValue::Object(vec![("a1", YAMLValue::Number(11.0))]), YAMLValue::Object(vec![("b1", YAMLValue::Number(12.0))])]))
        ])
    );
    assert!(parse_yaml_file("{el:1,}").is_err());
}

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
    let correct = "a: 1\n";
    let pair = Grammar::parse(Rule::key_value, "a: 1\n11111")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 5 );

    let pair = Grammar::parse(Rule::key_value, "");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::key_value, "{anything here}");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn value_rule_test() -> anyhow::Result< () > { 
    let correct = "  a: 1\n  b: 2\n";
    let pair = Grammar::parse(Rule::value, "\n  a: 1\n  b: 2\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 1 );
    assert_eq!( pair.as_span().end(), 15 );

    let pair = Grammar::parse(Rule::value, "\n\n\n");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn scalar_rule_test() -> anyhow::Result< () > { 
    let correct = "anything";
    let pair = Grammar::parse(Rule::scalar, "anything\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 8 );

    let pair = Grammar::parse(Rule::value, " ");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn number_rule_test() -> anyhow::Result< () > { 
    let correct = "75";
    let pair = Grammar::parse(Rule::number, "75\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 2 );

    let pair = Grammar::parse(Rule::number, "str");
    assert!(pair.is_err());


    Ok(())
}

#[test]
fn boolean_rule_test() -> anyhow::Result< () > { 
    let correct = "true";
    let pair = Grammar::parse(Rule::boolean, "true")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);

    let correct = "false";
    let pair = Grammar::parse(Rule::boolean, "false")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);

    let pair = Grammar::parse(Rule::boolean, "str");
    assert!(pair.is_err());
    Ok(())
}

#[test]
fn null_rule_test() -> anyhow::Result< () > { 
    let correct = "null";
    let pair = Grammar::parse(Rule::null, "null")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), correct);

    let pair = Grammar::parse(Rule::null, "str");
    assert!(pair.is_err());
    Ok(())
}


#[test]
fn string_rule_test() -> anyhow::Result< () > { 
    let correct = "str123";
    let pair = Grammar::parse(Rule::string, "str123")?;
    assert_eq!( pair.as_str(), correct);

    let correct = "str";
    let pair = Grammar::parse(Rule::string, "str 123")?;
    assert_eq!( pair.as_str(), correct);

    let pair = Grammar::parse(Rule::string, "[123]");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn line_end_rule_test() -> anyhow::Result< () > { 
    let correct = "";
    let pair = Grammar::parse(Rule::line_end, "\n\r123")?;
    assert_eq!( pair.as_str(), correct);

    let pair = Grammar::parse(Rule::line_end, "[123]");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn intennd_rule_test() -> anyhow::Result< () > { 
    let correct = "";
    let pair = Grammar::parse(Rule::indent, "  ")?;
    assert_eq!( pair.as_str(), correct);

    let pair = Grammar::parse(Rule::indent, "[123]");
    assert!(pair.is_err());

    Ok(())
}

