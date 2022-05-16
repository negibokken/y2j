use std::env;
use std::fs::File;
use std::io::Read;
use serde_yaml;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f: File = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let json_value: serde_json::Value = serde_yaml::from_str(&contents).unwrap();
    println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
}
