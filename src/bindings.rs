use crate::tokens::*;
use crate::statics::*;
use crate::scopes::*;

// built in bindings

pub struct MethodBindings {
    // list_names : Vec<String>,
    // dict_names : Vec<String>,
    // string_names : Vec<String>,
    // number_names : Vec<String>,
    // int_names : Vec<String>,
    // uint_names : Vec<String>,
    // float_names : Vec<String>,
    // bool_names : Vec<String>,
}

impl MethodBindings {
    pub fn new () -> MethodBindings {
        MethodBindings {}
    }
}

pub struct FuncBindings {}

impl FuncBindings {
    pub fn new () -> FuncBindings {
        FuncBindings {}
    }
    pub fn execute (operation : &str, inputs : Vec<&TokenData>) -> TokenData {
        match operation {
            "sqrt" => {if TypeEnforcer::validate((true, 1, vec!["number"])) {let x = match inputs[0] {
                TokenData::Float(f) => f.sqrt(),
                TokenData::UInt(u) => (*u as f64).sqrt(),
                TokenData::Int(i) => (*i as f64).sqrt(),
                _ => 0f64
            };if !x.is_finite() {return TokenData::Error("non-real result".to_owned());} if x.floor() == x {return TokenData::Int(x as i64);} return TokenData::Float(x);} TokenData::Error("bad input".to_owned())},

            "floor" => {if TypeEnforcer::validate((true, 1, vec!["number"])) {return match inputs[0] {
                TokenData::Float(f) => TokenData::Int(f.floor() as i64),
                TokenData::UInt(u) => TokenData::Int(*u as i64),
                TokenData::Int(i) => TokenData::Int(*i),
                _ => TokenData::Void,
            };} TokenData::Error("bad input".to_owned())},

            "ceil" => {if TypeEnforcer::validate((true, 1, vec!["number"])) {return match inputs[0] {
                TokenData::Float(f) => TokenData::Int(f.ceil() as i64),
                TokenData::UInt(u) => TokenData::Int(*u as i64),
                TokenData::Int(i) => TokenData::Int(*i),
                _ => TokenData::Void,
            };} TokenData::Error("bad input".to_owned())},

            _ => TokenData::Error("non-existing built in func operation".to_owned()),
        }
    }
}

pub struct OpBindings {}

impl OpBindings {
    pub fn execute (&self, opid : &str, args : &Vec<String>, data : &mut ScopeManager) -> Token {
        let dummyvec : Option<&Vec<Token>> = None;
        // data.dump();
        if opid == "console.log" {
            // println!("{}", data.deref_full(&args[0]));
            for item in match &data.deref_full(&args[0]).value {TokenData::List(l)=>&l,_=>dummyvec.unwrap()} {
                print!("{}, ", item);
            }
            print!("\n");
            return void_token();
        }
        return void_token();
    }
}

pub struct Builtins {
    pub methods : MethodBindings,
    pub funcs : FuncBindings,
    pub operations : OpBindings,
}

impl Builtins {
    pub fn new () -> Builtins {
        Builtins {
            methods : MethodBindings::new(),
            funcs : FuncBindings::new(),
            operations : OpBindings {},
        }
    }
}