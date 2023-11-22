use pest_derive::Parser;
use pest::Parser;
use pest::error::Error;
use std::fs;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub enum YAMLValue<'a> {
  Object(Vec<(&'a str, YAMLValue<'a>)>),
  Array(Vec<YAMLValue<'a>>),
  String(&'a str),
  Number(f64),
  Boolean(bool),
  Null,
}

pub fn serialize_yamlvalue(val: &YAMLValue) -> String {
  use YAMLValue::*;
  use std::string::String;
  let indentation = String::from("  ");
  match val {
      Object(o) => {
          let contents: Vec<_> = o
              .iter()
              .map(|(name, value)|
                   format!("{}: {}", name, serialize_yamlvalue(value)))
              .collect();
          format!("{}", contents.join("\n"))
      }
      Array(a) => {
        let contents: Vec<_> = a.iter().map(serialize_yamlvalue).collect();
        format!("\n{}{}", "  ", contents.join("\n  "))
    }
      String(s) => format!("{}", s),
      Number(n) => format!("{}", n),
      Boolean(b) => format!("{}", b),
      Null => format!("null"),
  }
}

pub fn read_file(file_path: &str) -> String {
  println!("Parsing data from file {}", String::from(file_path));

  let contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  // println!("File data:\n{contents}");
  contents
}


pub fn parse_yaml_file(file: &str) -> Result<YAMLValue, Error<Rule>> {

  let yaml = Grammar::parse(Rule::full_yaml, file)?.next().unwrap();

  use pest::iterators::Pair;

  fn parse_value(pair: Pair<Rule>) -> YAMLValue {
    println!("{:?}", pair);
      match pair.as_rule() {
          Rule::object => YAMLValue::Object(
              pair.into_inner()
                  .map(|pair| {
                      let mut inner_rules = pair.into_inner();
                      let name = inner_rules
                          .next()
                          .unwrap()
                          .as_str();
                      let value = parse_value(inner_rules.next().unwrap());
                      (name, value)
                  })
                  .collect(),
          ),
          Rule::subnode => YAMLValue::Array(pair.into_inner().map(parse_value).collect()),
          Rule::string => YAMLValue::String(pair.as_str()),
          Rule::number => YAMLValue::Number(pair.as_str().parse().unwrap()),
          Rule::boolean => YAMLValue::Boolean(pair.as_str().parse().unwrap()),
          Rule::null => YAMLValue::Null,
          Rule::helpnode => YAMLValue::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();
                    let name = inner_rules
                        .next()
                        .unwrap()
                        .as_str();
                    let value = parse_value(inner_rules.next().unwrap());
                    (name, value)
                })
                .collect(),
        ),
          Rule::full_yaml
          | Rule::EOI
          | Rule::key_value
          | Rule::scalar
          | Rule::value
          | Rule::indent
          | Rule::line_end => unreachable!(),
      }
  }
  Ok(parse_value(yaml))
}
