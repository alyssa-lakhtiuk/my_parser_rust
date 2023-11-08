use pest::Parser;
use anyhow::anyhow;
use my_parser_Lakhtiuk::*;

#[test]
fn basic_test() -> anyhow::Result< () > {
    let got = Grammar::parse(Rule::full_yaml, "---ghj:45")?;
    println!("{:?}", got);

    Ok(())
}

#[test]
fn brackets_test() -> anyhow::Result< () > {

    let pair = Grammar::parse(Rule::full_yaml, "---ghj:45")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "---ghj:45" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 9 );

    let pair = Grammar::parse(Rule::full_yaml, "");
    assert!(pair.is_err());

    let file_path = String::from("./test_files/first_test_file.txt");
    let content_from_file = read_file(file_path);
    //println!("{}", content_from_file);
    let pair = Grammar::parse(Rule::full_yaml, content_from_file.as_str())?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "---el0: 0" );

    Ok(())
}
