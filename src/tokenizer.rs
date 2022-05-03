// tokenizes code
use regex::Regex;
use lazy_static::*;
use crate::statics::*;
use crate::tokens::*;
use std::fs::read_to_string;

pub type TRes = Result<Vec<Token>, (i32, String)>;
pub type VTRes<T> = Result<T, (i32, String)>;

fn get_closing_grp (o : &str) -> String {
    match o {
        "{" => "}",
        "[" => "]",
        "(" => ")",
        _ => "",
    }.to_owned()
}

fn extract_grp (mut i : usize, tokens : &mut Vec<Token>) -> TRes {
    let mut depth : usize = 1;
    let oval = &match &tokens[i].value {TokenData::String(s)=>s.clone(),_=>String::new()};
    let lock = &get_closing_grp(oval);
    let mut f : Vec<Token> = Vec::new();
    i += 1;
    if lock == "" {
        printtoks(&tokens[i-2..]);
        return Err((2, "unopened group".to_owned()));
    }
    loop {
        if i >= tokens.len() {
            return Err((2, "group end not found".to_owned()));
        }
        if tokens[i].id == GRP {
            match &tokens[i].value {
                TokenData::String(s) => {if s == lock {depth -= 1} else if s == oval {depth += 1}},
                _ => {},
            };
            if depth == 0 {
                tokens.remove(i);
                break;
            }
        }
        f.push(tokens.remove(i));
    }
    return Ok(f);
}

fn preprocess (mut tokens : Vec<Token>) -> TRes {
    let mut i : usize = 0;
    let mut l : usize = tokens.len();
    // complex processing on multiple tokens
    loop {
        if i >= l {
            break;
        }
        if tokens[i].id == KEY {
            match &tokens[i].value {
                TokenData::String(s) => match &s[..] {
                    "func" => {},
                    "if" => {},
                    "else" => {},
                    "for" => {},
                    "while" => {},
                    _ => {},
                },
                _ => {},
            };
        } else if tokens[i].id == GRP {
            let opening = tokens[i].vstr();
            let x = preprocess(extract_grp(i, &mut tokens)?)?;
            if opening == "(" {
                tokens[i] = Token::new(PRN, TokenData::List(x));
            } else if opening == "[" {
                tokens[i] = Token::new(USB, TokenData::List(x));
            } else if opening == "{" {
                tokens[i] = Token::new(UCB, TokenData::List(x));
            }
            l = tokens.len();
        }
        i += 1;
    }
    return Ok(tokens);
}

pub fn tokenize (contents : String) -> TRes {
    lazy_static! {
        static ref NUMRE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
        static ref DECIRE : Regex = Regex::new(DECI_RE_PAT).unwrap();
        static ref OPRE : Regex = Regex::new(OP_RE_PAT).unwrap();
        static ref KEYRE : Regex = Regex::new(KEY_RE_PAT).unwrap();
        // static ref SYMRE : Regex = Regex::new(SYM_RE_PAT).unwrap();
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
        // groups
        } else if "()[]{}".find(chars[i]).is_some() {
            if word.len() > 0 {
                words.push(word);
                word = String::new();
            }
            words.push(String::from(chars[i]));
        // operators
        } else if "+-*/%&|^!=<>".find(chars[i]).is_some() {
            if word.len() > 0 {
                words.push(word);
                word = String::new();
            }
            word.push(chars[i]);
            if chars[i] == '-' && chars[i+1] == '>' {
                words.push("->".to_owned());
                word = String::new();
                i += 2;
                continue;
            }
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

    // let symcheck = ".|,|:|->".split('|');

    for word in words {
        // println!("{}, {}", word, SYMRE.is_match(&word));
        if word.starts_with('"') {
            f.push(Token::new(DAT, TokenData::String(word[1..word.len()-1].to_owned())));
        } else if word == ";" {
            f.push(Token::new(NLN, TokenData::String(word)));
        // } else if SYMRE.is_match(&word) {
        } else if ".:,->".find(&word).is_some() {
            f.push(Token::new(SYM, TokenData::String(word)));
        } else if "{}[]()".find(&word).is_some() {
            f.push(Token::new(GRP, TokenData::String(word)));
        } else if OPRE.is_match(&word) {
            f.push(Token::new(OPS, TokenData::String(word)));
        } else if KEYRE.is_match(&word) {
            f.push(Token::new(KEY, TokenData::String(word)));
        } else if NUMRE.is_match(&word) {
            f.push(Token::new(DAT, match word.find('.').is_some() {true=>TokenData::Float(word.parse().unwrap()),_=>TokenData::Int(word.parse().unwrap())}));
        } else {
            f.push(Token::new(DAT, TokenData::Name(word)));
        }
    }

    let mut i : usize = 0;
    let l : usize = f.len();

    // simple processing, occurrs on single tokens
    loop {
        if i >= l {
            break;
        }
        match &f[i].value {
            TokenData::Name(n) => match &n[..] {
                "true" => {f[i].value = TokenData::Bool(true);},
                "false" => {f[i].value = TokenData::Bool(false);},
                _ => {},
            },
            _ => {},
        };
        i += 1;
    }

    return preprocess(f);
}

pub fn tokenize_file (name : &str) -> TRes {
    return Ok(tokenize(match read_to_string(name) {
        Ok(c) => c,
        Err(_) => {return Err((0, String::from("failed to read file")))}
    })?);
}