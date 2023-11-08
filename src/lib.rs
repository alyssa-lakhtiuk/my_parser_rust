use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub fn read_file(file_path: String) -> String {
  println!("Parsing data from file {}", file_path);

  let contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  // println!("File data:\n{contents}");
  contents
}