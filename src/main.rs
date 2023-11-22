use pest::Parser;
use anyhow::anyhow;
use simple_yaml_parser_kma::*;
use clap::Parser as ClapParser;


#[derive(ClapParser, Debug)]
struct Cli {
    parse_file_path: String
}

fn main() -> anyhow::Result< () > {
    // let args = Cli::parse();
    // let file_path = &args.parse_file_path;
    // let content_from_file = read_file(file_path.as_str());
    let file_path = "./test_files/test_file2.txt";
    let content_from_file = read_file(file_path);
    println!("{:?}", content_from_file);

    // let got = Grammar::parse(Rule::full_yaml, "{ghj:45}")?;
    // println!("{:?}", got);
    let yaml: YAMLValue = parse_yaml_file(content_from_file.as_str()).expect("unsuccessful parse"); 

    println!("serialized");
    let indentafication = String::from("");
    println!("{}", serialize_yamlvalue(&yaml));

    Ok(())
}
