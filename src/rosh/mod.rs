pub mod prelude {
    pub use std::collections::HashMap;
}

use self::prelude::*;

pub mod builtin;
pub mod path;

#[derive(Debug)]
pub enum Token {
    Arg(String),
    Quoted((char, String)),
    Opp(String),
}

pub fn rosh() {
    let path_map = path::get_map().expect("unable to build path map");
    let builtin_map = builtin::get_map().expect("unable to load builtins");
    loop {
        self::prompt();
        if let Some(input) = self::read_input() {
            let tokens = parse_line(input);
            // if path_map.contains_key(&args[0]) {
            //     self::execute(path_map[&args[0]].clone(), args);
            // } else if builtin_map.contains_key(&args[0]) {
            //     let exec = builtin_map[&args[0]];
            //     exec(args);
            // } else {
            //     eprintln!("rosh: command not found: {}", &args[0])
            // }
        }
    }
}

pub fn parse_line(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars();
    let mut skip: usize = 0;

    while let Some(ch) = iter.next() {
        if skip > 1 {
            skip -= 1;
            println!("skip: '{}'", ch);
            continue;
        }
        match ch {
            ch if ch.is_whitespace() => continue,
            '"' | '\'' => {
                let s = iter.clone().take_while(|c| *c != ch).collect::<String>();
                // Won't work with: '"' or "'"
                skip += s.len() + 1;
                tokens.push(Token::Quoted((ch, s.to_owned())));
            },
            ch if ch.is_ascii() => {
                let mut iter = iter.clone();
                let mut s = iter
                    .take_while(|c| !c.is_whitespace())
                    .collect::<String>();
                s.insert(0, ch);
                skip += s.len();
                tokens.push(Token::Arg(s));
            },
            _ => {
                println!("c: '{}'", ch);
                continue;
            }
        }
    }
    println!("{:?}", tokens);
    tokens
}

pub fn read_input() -> Option<String> {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    buf.pop();
    if buf.len() < 1 {
        None
    } else {
        Some(buf)
    }
}

pub fn prompt() {
    let prompt = std::env::var("PS1").unwrap_or("rosh> ".to_string());
    eprint!("{}", prompt);
}

pub fn execute(path: String, args: Vec<String>) {
    let mut args = args.clone();
    args.remove(0);
    let mut child = std::process::Command::new(path).args(args).spawn().unwrap();
    let _ = child.wait();
}
