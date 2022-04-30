#![allow(dead_code, unused_variables)]
use std::fmt;
use std::collections::HashMap;
use crate::statics::*;


// defines tokens

/**
 * Token is used when a token could be pulled from TokenData
 * otherwise TokenData is used
 */

#[derive(Clone, Debug)]
pub enum TokenData {
    Name(String),
    String(String),
    // data, types
    List(Vec<Token>, String),
    // data, types
    Dict(HashMap<String, Token>, String),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    // args, return type, code
    Function(Vec<TokenData>, Box<TokenData>, Vec<Token>),
    // name, statics, instance
    Class(String, HashMap<String, Token>, HashMap<String, Token>),
    // name, type, default
    Pair(String, Box<TokenData>, Box<TokenData>),
    Void
}

pub fn void_data () -> TokenData {
    return TokenData::Void;
}

pub fn void_token () -> Token {
    return Token::new(DAT, 0, void_data());
}

// complex construction
impl TokenData {
    pub fn list_new (tokens : Vec<Token>) -> TokenData {
        TokenData::List(tokens, String::from("list"))
    }
    pub fn dict_new (data : HashMap<String, Token>) -> TokenData {
        TokenData::Dict(data, String::from("dict"))
    }
}

// type matching funcs
impl TokenData {
    fn match_func_type (&self, test : &str) -> bool {
        return false;
    }
    fn match_list_type (&self, test : &str) -> bool {
        if test == "list" {
            return true;
        }
        if !test.starts_with("list") {
            return false;
        }
        match self {
            TokenData::List(v,t) => {
                if test == t || test == "list<any>" {return true;}
                println!("{}", test);
                let mt = &test[5..test.len()-1];
                for item in v {
                    if !item.value.match_type(mt) {
                        return false;
                    }
                }
                return true;
            },
            _ => {return false;}
        };
    }
    fn match_type_str (test : &str, target : &str) -> bool {
        return false;
    }
    fn match_dict_type (&self, test : &str) -> bool {
        if test == "dict" {
            return true;
        }
        if !test.starts_with("dict") {
            return false;
        }
        match self {
            TokenData::Dict(d,t) => {
                if test == t || test == "dict<any,any>" {return true;}
                let types : Vec<&str> = test[5..test.len()-1].split(",").collect();
                for (key, value) in d {
                    if !(TokenData::match_type_str(key, types[0]) && value.value.match_type(types[1])) {
                        return false;
                    }
                }
                return true;
            },
            _ => {return false;}
        };
    }
    pub fn match_type (&self, test : &str) -> bool {
        if test == "any" {
            return true;
        }
        let mut depth : usize = 0;
        if test.find(|c : char| {if c == '|' && depth == 0 {return true} if c == '<' {depth += 1} if c == '>' {depth -= 1} return false;}).is_some() {
            depth = 0;
            for l in test.split(|c : char| {if c == '|' && depth == 0 {return true} if c == '<' {depth += 1} if c == '>' {depth -= 1} return false;}) {
                println!("{}", l);
                if self.match_type(l) {
                    return true;
                }
            }
            return false;
        }
        match self {
            TokenData::List(_,_) => self.match_list_type(test),
            TokenData::Dict(_,_) => self.match_dict_type(test),
            TokenData::Bool(_) => test == "bool",
            TokenData::String(_) => test == "str",
            TokenData::Int(_) => test == "int" || test == "number",
            TokenData::UInt(_) => test == "uint" || test == "number",
            TokenData::Float(_) => test == "float" || test == "number",
            TokenData::Name(_) => false,
            TokenData::Pair(_, _, _) => false,
            TokenData::Function(_, _, _) => self.match_func_type(test),
            TokenData::Class(s, _, _) => test == s,
            TokenData::Void => test == "void"
        }
    }
}

impl TokenData {
    fn unwrap_bool (data : TokenData) -> bool {
        return match data {TokenData::Bool(b)=>b,_=>false};
    }
    fn unwrap_int (data : TokenData) -> i64 {
        return match data {TokenData::Int(b)=>b,_=>0};
    }
    fn unwrap_float (data : TokenData) -> f64 {
        return match data {TokenData::Float(b)=>b,_=>0f64};
    }
    fn unwrap_string (data : TokenData) -> String {
        return match data {TokenData::String(b)=>b.clone(),_=>String::from("")};
    }
    fn unwrap_list (data : TokenData) -> Vec<Token> {
        return match data {TokenData::List(b,_)=>b.clone(),_=>vec![]};
    }
    fn unwrap_dict (data : TokenData) -> HashMap<String, Token> {
        return match data {TokenData::Dict(b,_)=>b.clone(),_=>HashMap::new()};
    }
}

// list manipulation
impl TokenData {
    fn push_list (&mut self, token : Token) {
        match self {
            TokenData::List(v,_) => {v.push(token)},
            _ => {}
        };
    }
    fn pop_list (&mut self) -> Token {
        match self {
            TokenData::List(v,_) => v.pop().unwrap_or(void_token()),
            _ => void_token()
        }
    }
    fn get_list (&self, index : usize) -> Token {
        match self {
            TokenData::List(v,_) => v[index].clone(),
            _ => void_token()
        }
    }
    fn insert_list (&mut self, index : usize, token : Token) {
        match self {
            TokenData::List(v,_) => {v.insert(index, token)},
            _ => {}
        };
    }
    fn remove_list (&mut self, index : usize) -> Token {
        match self {
            TokenData::List(v,_) => v.remove(index),
            _ => void_token()
        }
    }
}

// dict manipulation
impl TokenData {
    fn insert_dict (&mut self, key : String, value : Token) {
        match self {
            TokenData::Dict(d,_) => {d.insert(key, value);},
            _ => {}
        };
    }
    fn remove_dict (&mut self, key : String) -> Token {
        match self {
            TokenData::Dict(d,_) => d.remove(&key).unwrap_or(void_token()),
            _ => void_token()
        }
    }
    fn get_dict (&self, key : String) -> Token {
        match self {
            TokenData::Dict(d,_) => d.get(&key).unwrap_or(&void_token()).clone(),
            _ => void_token()
        }
    }
    fn contains_dict (&self, key : String) -> bool {
        match self {
            TokenData::Dict(d,_) => d.contains_key(&key),
            _ => false
        }
    }
}

impl fmt::Display for TokenData {
    fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let v : String = match self {
            TokenData::Void => String::from("Void"),
            TokenData::List(v,_) => format!("{:?}", v),
            // TokenData::List(_) => String::from("List"),
            TokenData::Dict(_,_) => String::from("Dict"),
            TokenData::Function(_,_,_) => String::from("Function"),
            TokenData::Class(_,_,_) => String::from("Class"),
            TokenData::Pair(_,_,_) => String::from("Pair"),
            TokenData::Bool(b) => b.to_string(),
            TokenData::Int(i) => i.to_string(),
            TokenData::Float(f) => f.to_string(),
            TokenData::String(s) => s.to_string(),
            TokenData::Name(n) => n.to_string(),
            TokenData::UInt(u) => u.to_string()
        };
        write!(f, "{}", v)
    }
}

#[derive(Clone)]
pub struct Token {
    pub id : u8, // identifies the token as a general type
    pub altid : u8, // specific id
    pub value : TokenData, // containing data
}

impl Token {
    pub fn new (id : u8, aid : u8, value : TokenData) -> Token {
        Token {
            id : id,
            altid : aid,
            value : value,
        }
    }

    pub fn list_get (&self, index : usize) -> Token {
        return Token::new(DAT, 0, TokenData::Void);
    }
    pub fn list_set (&mut self, index : usize, value : Token) {}

    pub fn list_push (&mut self, value : Token) {
        self.value.push_list(value);
    }

    pub fn dict_get (&self, key : TokenData) -> Token {
        return Token::new(DAT, 0, TokenData::Void);
    }
    pub fn dict_set (&mut self, key : TokenData, value : Token) {}

    pub fn string_get (&self, index : usize) -> Token {
        let s : String = match &self.value {
            TokenData::String(v) => v.clone(),
            _ => String::from("")
        };
        return Token::new(DAT, 0, match s.get(index..index) {Some(st) => TokenData::String(st.to_string()), None => TokenData::Void});
    }
}

impl fmt::Display for Token {
    fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", IDMAP[self.id as usize], self.value)
    }
}

impl fmt::Debug for Token {
    fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
        .field("id", &IDMAP[self.id as usize])
        .field("altid", &self.altid)
        .field("value", &self.value)
        .finish()
    }
}

// pub fn test () {
//     let mut t : Token = Token::new(DAT, TokenData::List(Vec::new()));
//     t.list_push(Token::new(DAT, TokenData::Void));

//     println!("{}", t);
// }