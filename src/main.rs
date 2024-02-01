use std::{env, io};
use std::process::Command;
use json;

const CLEAR_CLIP_DELAY: u8 = 10;

#[derive(Debug, Clone, Copy)]
enum Request {
    Password,
    Username,
    Card,
}

fn display_options(&request: &Request, i: usize, result: json::JsonValue) -> () {
    // TODO
    // make the print prettier
    let slice;
    let statements = match request {
        Request::Password => (&result["name"], &result["login"]["username"]),
        Request::Username => (&result["name"], &result["login"]["username"]),
        Request::Card => {
            let s = &result["card"]["number"].as_str().expect("Invalid string found");
            // get last four characters
            let idx = s.char_indices().rev().nth(3).map(|(i, _)| i)
                .expect("String too short");
            slice = json::JsonValue::from(String::from(&s[idx..]));
            (&result["name"], &slice)
            },
    };
    println!("\t{i}:\t{}\t{}", &statements.0, &statements.1);
}

fn index_selection(n: usize) -> usize {
    println!("Select an option [0-{}].", n - 1);
    let index: usize;
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(value) => {if value < n { index = value; break }},
            Err(_) => {println!("Index entered is not valid.")}
        }
    }
    index
}

fn process_output(&request: &Request, object: json::JsonValue) -> String {
    let output = match request {
        Request::Password => &object["login"]["password"],
        Request::Username => &object["login"]["username"],
        Request::Card => {
            println!("Copying card number and displaying addtional info.");
            println!("\tExpiry Date: {}/{}\tCVV: {}",
                     &object["card"]["expMonth"],
                     &object["card"]["expYear"],
                     &object["card"]["code"]);
            &object["card"]["number"]
        },
    };
    output.to_string()
}

fn copy_to_clipboard(output: &str, delay: u8) {
    let clipper = match env::consts::OS {
        "linux" => "xclip -selection clipboard",
        "macos" => "pbcopy",
        _ => unimplemented!(),
    };
    // run in background process
    Command::new("bash")
        .arg("-c")
        .arg(format!("(sleep {} && echo -n '{}' | {}) &", delay, output, clipper))
        .status()
        .expect("Failed to copy to clipboard");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let request = match &args[2] as &str {
        "pass" => Request::Password,
        "name" => Request::Username,
        "card" => Request::Card,
        &_ => panic!("Invalid request passed from bash script")
    };

    let data = json::parse(&args[1]).expect("Unable to parse JSON");
    let n = data.len();
    if n == 0 {
        println!("No results for your search.");
        return
    }
    println!("There are {} results for your search.", n);
    for (i, result) in data.members().enumerate() {
        display_options(&request, i, result.clone());
    }

    let index = match n {
        1 => 0,
        _ => index_selection(n),
    };

    let object = data.members().nth(index).unwrap();
    let output = process_output(&request, object.clone());
    copy_to_clipboard(&output, 0);
    println!("Added to clipboard, will be cleared in {} seconds.", CLEAR_CLIP_DELAY);
    copy_to_clipboard("", CLEAR_CLIP_DELAY); // clear clipboard after delay
}
