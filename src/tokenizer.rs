// tokenizes code
use regex::Regex;
use lazy_static::*;
use crate::statics::*;
use crate::tokens::*;
use std::fs::read_to_string;

pub type TRes = Result<Vec<Token>, (i32, String)>;

fn preprocess (tokens : Vec<Token>) -> TRes {
    return Ok(tokens);
}

pub fn tokenize (contents : String) -> TRes {
    lazy_static! {
        static ref NUMRE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
        static ref DECIRE : Regex = Regex::new(DECI_RE_PAT).unwrap();
        static ref OPRE : Regex = Regex::new(OP_RE_PAT).unwrap();
        static ref KEYRE : Regex = Regex::new(KEY_RE_PAT).unwrap();
    };
    let chars : Vec<char> = contents.chars().collect();

    let mut i : usize = 0;
    let len : usize = chars.len();

    let mut words : Vec<String> = Vec::new();

    let mut word : String = String::new();

    'outer:  loop {
        if i >= len {
            break;
        }
        
        // newlines
        if chars[i] == ';' {
            if word.len() > 0 {
                words.push(word);
                word = String::new();
            }
            words.push(String::from(chars[i]));
        // strings
        } else if chars[i] == '"' {
            if word.len() > 0 {
                words.push(word);
                word = String::new();
            }
            word.push(chars[i]);
            i += 1;
            loop {
                if i >= len {
                    return Err((1, String::from("unclosed string")));
                }
                word.push(chars[i]);
                if chars[i] == '"' && (chars[i-1] != '\\' || chars[i-2] == '\\') {
                    break;
                }
                i += 1;
            }
            words.push(word);
            word = String::new();
        // whitespace (ignored)
        } else if chars[i] == ' ' || chars[i] == '\n' {
            if word.len() > 0 {
                words.push(word);
                word = String::new();
            }
        // operators
        } else if "+-*/%&|^!=<>".find(chars[i]).is_some() {
            if word.len() > 0 {
                words.push(word);
                word = String::new();
            }
            word.push(chars[i]);
            if i < len - 1 {
                if chars[i+1] == '=' {
                    word.push(chars[i+1]);
                    i += 1;
                } else if chars[i+1] == chars[i] {
                    if "&|+-*".find(chars[i]).is_some() {
                        word.push(chars[i]);
                        i += 1;
                    // comment
                    } else if chars[i] == '/' {
                        loop {
                            if i >= len {
                                break 'outer;
                            }

                            if chars[i] == '\n' {
                                break;
                            }
                            // word.push(chars[i]);

                            i += 1;
                        }
                        word = String::new();
                        i += 1;
                        continue;
                    }
                }
            }
            words.push(word);
            word = String::new();
        // dot
        } else if chars[i] == '.' && !DECIRE.is_match(&word) {
            if word.len() > 0 {
                words.push(word);
                words.push(String::from(chars[i]));
                word = String::new();
            }
        // not handled specially
        } else {
            word.push(chars[i]);
        }

        i += 1;
    }

    if word.len() > 0 {
        words.push(word);
    }

    let mut f : Vec<Token> = Vec::new();

    for word in words {
        if word.starts_with('"') {
            f.push(Token::new(DAT, 0, TokenData::String(word[1..word.len()-1].to_owned())));
        } else if word == ";" {
            f.push(Token::new(NLN, 0, TokenData::String(word)));
        } else if word == "." {
            f.push(Token::new(SYM, 0, TokenData::String(word)));
        } else if OPRE.is_match(&word) {
            f.push(Token::new(OPS, 0, TokenData::String(word)));
        } else if KEYRE.is_match(&word) {
            f.push(Token::new(KEY, 0, TokenData::String(word)));
        } else {
            f.push(Token::new(DAT, 0, TokenData::Name(word)));
        }
    }

    return preprocess(f);
}

pub fn tokenize_file (name : &str) -> TRes {
    return Ok(tokenize(match read_to_string(name) {
        Ok(c) => c,
        Err(_) => {return Err((0, String::from("failed to read file")))}
    })?);
}