use simple_yaml_parser_kma::*;
use clap::Parser as ClapParser;


#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// path to the parsing file
    parse_file_path: String
}

fn main() -> anyhow::Result< () > {
    let args = Cli::parse();
    let file_path = &args.parse_file_path;
    let content_from_file = read_file(file_path.as_str());

    let yaml: YAMLValue = parse_yaml_file(content_from_file.as_str()).expect("unsuccessful parse"); 

    println!("serialized");
    println!("{}", serialize_yamlvalue(&yaml));

    Ok(())
}
