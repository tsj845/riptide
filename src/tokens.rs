#![allow(dead_code, unused_variables)]
use std::fmt;
use std::collections::HashMap;
use crate::statics::*;
use terminal_size::{Width, Height, terminal_size};


// defines tokens

// pub fn printtoks (toks : &Vec<Token>) {
fn printtoks_internal (toks : &[Token], limit : usize, do_lim : bool) {
    print!("[ ");
    let mut count : usize = 0;
    let max : usize;
    if let Some((Width(w), Height(_))) = terminal_size() {
        max = w as usize - 21;
    } else {
        max = limit;
        // max = 25;
    }
    for tok in toks {
        let mut s = tok.ostr();
        // println!("\nS:{:?}", s.chars().collect::<Vec<_>>());
        if do_lim {
            let st = &s[1..].find('(').unwrap()+1;
            let e = s.len()-3;
            let c = std::cmp::min(e-st, 17);
            let r = &(s[st..st+c].to_owned()+match c==17&&(e-st)>17 {true=>"...",false=>""});
            s.replace_range(st..e, r);
        }
        count += s.len() + 2;
        print!("{}, ", s);
        if count >= max {
            print!("\n  ");
            count = 0;
        }
    }
    println!(" ]");
}
pub fn printtoks (toks : &[Token]) {
    printtoks_internal(toks, 25, true);
}
pub fn printtoks_nolim (toks : &[Token]) {
    printtoks_internal(toks, 1000, false);
}

/**
 * Token is used when a token could be pulled from TokenData
 * otherwise TokenData is used
 */

#[derive(Debug, Clone)]
pub enum TokenData {
    Name(String),
    String(String),
    // data, types
    List(Vec<Token>),
    // data, types
    Dict(HashMap<String, Token>),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    // args, return type, code
    Function(Vec<TokenData>, String, Vec<Token>),
    // name, statics, instance
    Class(String, HashMap<String, Token>, HashMap<String, Token>),
    // name, type, default
    Pair(String, String, Box<TokenData>),
    Void,
    // error
    Error(String),
    // opid, supplied args
    OPCall(String, Vec<String>)
}

pub fn void_data () -> TokenData {
    return TokenData::Void;
}

pub fn void_token () -> Token {
    // return Token::new(DAT, 0, void_data());
    return Token::new(DAT, void_data());
}

// // complex construction
// impl TokenData {
//     pub fn list_new (tokens : Vec<Token>) -> TokenData {
//         TokenData::List(tokens, String::from("list"))
//     }
//     pub fn dict_new (data : HashMap<String, Token>) -> TokenData {
//         TokenData::Dict(data, String::from("dict"))
//     }
// }

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
            TokenData::List(v) => {
                if test == "list<any>" {return true;}
                // println!("{}", test);
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
            TokenData::Dict(d) => {
                if test == "dict<any,any>" {return true;}
                let mut depth : usize = 0;
                let types : Vec<&str> = test[5..test.len()-1].split(|c : char| {if c==','&& depth==0{return true;}if c=='('||c=='<'{depth+=1;}if c==')'||c=='>'{depth-=1;}return false;}).collect();
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
    pub fn match_constraint (&self, test : &str) -> bool {
        if test.starts_with("str") {
            return match self {
                //.collect::<Vec<_>>().iter()
                TokenData::String(s) => test[4..test.len()-1].split("|").find(|item:&&str|{item==s}).is_some(),
                _ => false,
            };
        }
        println!("catchall");
        return false;
    }
    pub fn match_type (&self, test : &str) -> bool {
        if test == "any" {
            return true;
        }
        let mut depth : usize = 0;
        if test.find(|c : char| {if c == '|' && depth == 0 {return true} if c == '<' || c == '(' {depth += 1} if c == '>' || c == ')' {depth -= 1} return false;}).is_some() {
            depth = 0;
            for l in test.split(|c : char| {if c == '|' || c == '(' && depth == 0 {return true} if c == '<' || c == ')' {depth += 1} if c == '>' {depth -= 1} return false;}) {
                println!("{}", l);
                if self.match_type(l) {
                    return true;
                }
            }
            return false;
        }
        match self {
            TokenData::List(_) => self.match_list_type(test),
            TokenData::Dict(_) => self.match_dict_type(test),
            TokenData::Bool(_) => test == "bool",
            TokenData::String(_) => test == "str" && self.match_constraint(test),
            TokenData::Int(_) => test == "int" || test == "number",
            TokenData::UInt(_) => test == "uint" || test == "number",
            TokenData::Float(_) => test == "float" || test == "number",
            // TokenData::Name(_) => false,
            // TokenData::Pair(_, _, _) => false,
            TokenData::Function(_, _, _) => self.match_func_type(test),
            TokenData::Class(s, _, _) => test == s,
            // TokenData::Error(_) => false, 
            TokenData::Void => test == "void",
            _ => false
        }
    }
}

pub struct TypeEnforcer {}

impl TypeEnforcer {
    pub fn validate (data : (bool, u64, Vec<&str>)) -> bool {
        return true;
    }
}

impl TokenData {
    // pub fn unwrap_bool (data : TokenData) -> bool {
    //     return match data {TokenData::Bool(b)=>b,_=>false};
    // }
    pub fn unwrap_bool (&self) -> bool {
        return match self {TokenData::Bool(b)=>*b,_=>false};
    }
    pub fn unwrap_int (data : TokenData) -> i64 {
        return match data {TokenData::Int(b)=>b,_=>0};
    }
    pub fn unwrap_float (data : TokenData) -> f64 {
        return match data {TokenData::Float(b)=>b,_=>0f64};
    }
    // pub fn unwrap_string (data : TokenData) -> String {
    //     return match data {TokenData::String(b)=>b.clone(),_=>String::from("")};
    // }
    pub fn unwrap_string (&self) -> String {
        return match self {TokenData::String(b)=>b.clone(),_=>String::new()};
    }
    pub fn string_ref (&self) -> &str {
        return match self {TokenData::String(b)=>b,_=>""};
    }
    pub fn unwrap_list (data : TokenData) -> Vec<Token> {
        return match data {TokenData::List(b)=>b.clone(),_=>vec![]};
    }
    pub fn unwrap_dict (data : TokenData) -> HashMap<String, Token> {
        return match data {TokenData::Dict(b)=>b.clone(),_=>HashMap::new()};
    }
    pub fn pair_ref (&self) -> (&str, &str, &Box<TokenData>) {
        return match self {TokenData::Pair(n,t,d)=>(n,t,d),_=>{unreachable!()}};
    }
}

// list manipulation
impl TokenData {
    fn push_list (&mut self, token : Token) {
        match self {
            TokenData::List(v) => {v.push(token)},
            _ => {}
        };
    }
    fn pop_list (&mut self) -> Token {
        match self {
            TokenData::List(v) => v.pop().unwrap_or(void_token()),
            _ => void_token()
        }
    }
    fn get_list (&self, index : usize) -> Token {
        match self {
            TokenData::List(v) => v[index].clone(),
            _ => void_token()
        }
    }
    fn insert_list (&mut self, index : usize, token : Token) {
        match self {
            TokenData::List(v) => {v.insert(index, token)},
            _ => {}
        };
    }
    fn remove_list (&mut self, index : usize) -> Token {
        match self {
            TokenData::List(v) => v.remove(index),
            _ => void_token()
        }
    }
}

// dict manipulation
impl TokenData {
    fn insert_dict (&mut self, key : String, value : Token) {
        match self {
            TokenData::Dict(d) => {d.insert(key, value);},
            _ => {}
        };
    }
    fn remove_dict (&mut self, key : String) -> Token {
        match self {
            TokenData::Dict(d) => d.remove(&key).unwrap_or(void_token()),
            _ => void_token()
        }
    }
    fn get_dict (&self, key : String) -> Token {
        match self {
            TokenData::Dict(d) => d.get(&key).unwrap_or(&void_token()).clone(),
            _ => void_token()
        }
    }
    fn contains_dict (&self, key : String) -> bool {
        match self {
            TokenData::Dict(d) => d.contains_key(&key),
            _ => false
        }
    }
}

impl fmt::Display for TokenData {
    fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let v : String = match self {
            TokenData::Void => String::from("Void"),
            TokenData::List(v) => format!("List({:?})", v),
            // TokenData::List(_) => String::from("List"),
            TokenData::Dict(_) => String::from("Dict"),
            TokenData::Function(_,_,_) => String::from("Function"),
            TokenData::Class(_,_,_) => String::from("Class"),
            TokenData::Pair(_,_,_) => String::from("Pair"),
            TokenData::Bool(b) => format!("Bool({})", b),
            TokenData::Int(i) => format!("Int({})", i),
            TokenData::Float(f) => format!("Float({})", f),
            TokenData::String(s) => format!("String(\"{}\")", s),
            TokenData::Name(n) => format!("Name({})", n),
            TokenData::UInt(u) => format!("UInt({})", u),
            TokenData::Error(s) => format!("Error(\"{}\")", s),
            TokenData::OPCall(_,_) => String::from("OPCALL"),
        };
        write!(f, "{}", v)
    }
}

#[derive(Clone)]
pub struct Token {
    pub id : u8, // identifies the token as a general type
    // pub altid : u8, // specific id
    pub value : TokenData, // containing data
}

impl Token {
    // pub fn new (id : u8, aid : u8, value : TokenData) -> Token {
    pub fn new (id : u8, value : TokenData) -> Token {
        Token {
            id : id,
            // altid : aid,
            value : value,
        }
    }

    pub fn vstr (&self) -> String {
        return match &self.value {TokenData::String(s)=>s.clone(),_=>String::new()};
    }

    // pub fn read_altid (&self) -> AltId {
    //     return AltId::decode(self.id, self.altid);
    // }

    pub fn list_get (&self, index : usize) -> Token {
        // return Token::new(DAT, 0, TokenData::Void);
        return Token::new(DAT, TokenData::Void);
    }
    pub fn list_set (&mut self, index : usize, value : Token) {}

    pub fn list_push (&mut self, value : Token) {
        self.value.push_list(value);
    }

    pub fn dict_get (&self, key : TokenData) -> Token {
        // return Token::new(DAT, 0, TokenData::Void);
        return Token::new(DAT, TokenData::Void);
    }
    pub fn dict_set (&mut self, key : TokenData, value : Token) {}

    pub fn string_get (&self, index : usize) -> Token {
        let s : String = match &self.value {
            TokenData::String(v) => v.clone(),
            _ => String::from("")
        };
        // return Token::new(DAT, 0, match s.get(index..index) {Some(st) => TokenData::String(st.to_string()), None => TokenData::Void});
        return Token::new(DAT, match s.get(index..index) {Some(st) => TokenData::String(st.to_string()), None => TokenData::Void});
    }
}

impl fmt::Display for Token {
    fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc : usize;
        if self.id == OPCALL {
            loc = 0;
        } else {
            loc = self.id as usize + 1;
        }
        write!(f, "({}, {})", IDMAP[loc], self.value)
    }
}

impl Token {
    pub fn ostr (&self) -> String {
        let loc : usize;
        if self.id == OPCALL {
            loc = 0;
        } else {
            loc = self.id as usize + 1;
        }
        return format!("({}, {})", IDMAP[loc], self.value);
    }
}

impl fmt::Debug for Token {
    fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc : usize;
        if self.id == OPCALL {
            loc = 0;
        } else {
            loc = self.id as usize + 1;
        }
        f.debug_struct("Token")
        .field("id", &IDMAP[loc])
        .field("value", &self.value)
        .finish()
    }
}

// pub fn test () {
//     let mut t : Token = Token::new(DAT, TokenData::List(Vec::new()));
//     t.list_push(Token::new(DAT, TokenData::Void));

//     println!("{}", t);
// }