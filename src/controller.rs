// does everything related to running the interpreter
use crate::tokens::*;
#[allow(unused_imports)]
use crate::tokenizer::{tokenize, tokenize_file};
use crate::runner::Runner;
use crate::statics::DAT;


fn start (tokens : Vec<Token>) {
    let mut runner : Runner = Runner::new();

    println!("{}", match runner.run(tokens) {Ok(msg) => msg,Err(msg) => msg});
}

fn test_1 () {
    let mut t = Token::new(DAT, 0, TokenData::list_new(Vec::new()));
    t.list_push(Token::new(DAT, 0, TokenData::Bool(true)));
    println!("{}, {}", t.value.match_type("list<any>"), t.value.match_type("list<str|bool>"));
}

pub fn run (args : Vec<String>) {
    test_1();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("aborting: no target file supplied");
        return;
    }

    match tokenize_file(&args[1]) {
        Ok(t) => {start(t)},
        Err((partid, msg)) => {match partid {
            0 => {println!("error during file processing:")},
            1 => {println!("error during initial tokenization:")},
            2 => {println!("error during token preprocessing:")},
            _ => {}
        };println!("{}", msg)}
    }
}