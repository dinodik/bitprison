use std::{env, io};
use std::process::Command;
use json;

// TODO 
// - loop on index failure
// - branch for pass/user/card inputs
// - allow for continuation after copy (view other IDs or get different element of current)
// - clear clipboard and terminal after timeout

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = json::parse(&args[1]).expect("Unable to parse JSON");
    println!("There are {} results for your search.", data.len());
    for (i, result) in data.members().enumerate() {
        println!("\t{i}:\t{}\t{}", &result["name"], &result["login"]["username"]);
    }
    println!("Select an option [0-{}].", data.len() - 1);
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number");
    let output: String = data.members()
        .collect::<Vec<_>>()
        .get(index)
        .expect("Index out of bounds")["login"]["username"]
        .to_string();
    Command::new("sh")
        .arg("-c")
        .arg(format!("echo '{}' | xclip -selection clipboard", output))
        .status()
        .expect("Failed to copy to clipboard");
    println!("{}", "Copied value into clipboard.");
}
