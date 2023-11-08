use pest::Parser;
use anyhow::anyhow;
use my_parser_Lakhtiuk::*;

fn main() -> anyhow::Result< () > {
    let got = Grammar::parse(Rule::full_yaml, "{ghj:45}")?;
    println!("{:?}", got);

    let file_path = String::from("./test_files/first_test_file.txt");
    read_file(file_path);
    Ok(())
}
