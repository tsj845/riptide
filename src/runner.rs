#![allow(non_upper_case_globals)]
#[allow(unused_imports)]
use crate::statics::*;
use crate::tokens::*;
use crate::scopes::*;
use crate::bindings::*;
use std::collections::HashMap;

pub type EvalResult = Result<Vec<Token>, String>;
pub type EvalResultO = Result<Token, String>;
pub type EvalResultN = Result<(), String>;
pub const EvalOk : Result<(),String> = Ok(());

// executes code
#[allow(dead_code)]
pub struct Runner {
    scopes : ScopeManager,
    binds : Builtins,
}

impl Runner {
    pub fn new () -> Runner {
        Runner {
            scopes : ScopeManager::new(),
            binds : Builtins::new(),
        }
    }
    #[allow(unreachable_code)]
    pub fn do_op (&self, left : Token, op : Token, right : Token) -> EvalResultO {
        // let f : Token;
        match match &op.value {TokenData::String(s)=>&s,_=>""} {
            "+" => {
                let t1 = match &left.value {TokenData::String(_)=>1,TokenData::Int(_)=>2,TokenData::UInt(_)=>3,TokenData::Float(_)=>4,_=>0};
                let t2 = match &right.value {TokenData::String(_)=>1,TokenData::Int(_)=>2,TokenData::UInt(_)=>3,TokenData::Float(_)=>4,_=>0};
                if t1 == 1 {
                    if t2 != 1 {
                        return Err("incompatable types for concat".to_owned());
                    }
                    return Ok(Token::new(DAT, TokenData::String(match &left.value {TokenData::String(s)=>s.clone(),_=>String::new()} + match &right.value {TokenData::String(s)=>&s,_=>""})));
                } else if t1 > 0 && t2 > 0 {
                    if t2 != t1 {
                        return Err("can't add numbers of differing types, consider using a cast".to_owned());
                    }
                    let r : f64 = match &left.value {TokenData::Float(x)=>*x,TokenData::Int(x)=>(*x as f64),TokenData::UInt(x)=>(*x as f64),_=>0f64} + match&right.value {TokenData::Float(x)=>*x,TokenData::Int(x)=>(*x as f64),TokenData::UInt(x)=>(*x as f64),_=>0f64};
                    if t1 == 3 {
                        return Ok(Token::new(DAT, TokenData::UInt(r as u64)));
                    } else if t1 == 2 {
                        return Ok(Token::new(DAT, TokenData::Int(r as i64)));
                    }
                    return Ok(Token::new(DAT, TokenData::Float(r)));
                } else {
                    return Err("addition not supported for given type".to_owned());
                }
            },
            "-" => {
                let t1 = match &left.value {TokenData::Int(_)=>1,TokenData::UInt(_)=>2,TokenData::Float(_)=>3,_=>0};
                let t2 = match &right.value {TokenData::Int(_)=>1,TokenData::UInt(_)=>2,TokenData::Float(_)=>3,_=>0};
                if t1 == 0 || t2 == 0 {
                    return Err("subtraction not supported for given type".to_owned());
                }
                if t1 != t2 {
                    return Err("can't subtract numbers of differing types, consider using a cast".to_owned());
                }
                let mut r : f64 = match &left.value {TokenData::Float(x)=>*x,TokenData::Int(x)=>(*x as f64),TokenData::UInt(x)=>(*x as f64),_=>0f64} - match&right.value {TokenData::Float(x)=>*x,TokenData::Int(x)=>(*x as f64),TokenData::UInt(x)=>(*x as f64),_=>0f64};
                if t1 == 1 {
                    return Ok(Token::new(DAT, TokenData::Int(r as i64)));
                } else if t1 == 2 {
                    if r < 0f64 {
                        r = std::f64::MAX - r;
                    }
                    return Ok(Token::new(DAT, TokenData::UInt(r as u64)));
                } else {
                    return Ok(Token::new(DAT, TokenData::Float(r)));
                }
            },
            _ => {return Err("unknown operator".to_owned())},
        };
        return Err("escaped match statement".to_owned());
        // return Ok(f);
    }
    pub fn eval_func (&mut self, ptr : Token, call : Token) -> EvalResultO {
        // let ptrval = match &ptr.value{TokenData::String(s)=>&s,_=>""};
        println!("{}", ptr);
        let fun : (Vec<TokenData>, String, Vec<Token>) = match &self.scopes.heap.get(&match ptr.value{TokenData::String(s)=>s,_=>String::new()}).unwrap().value {TokenData::Function(a, r, v)=>(a.clone(),r.clone(),v.clone()),_=>(vec![],String::new(),vec![])};
        self.scopes.new_scope();
        for a in &fun.0 {
            match &a {
                TokenData::Pair(n, _, d) => {
                    self.scopes.insert(&n, Flag::new());
                    let dat = Box::into_raw(d.clone());
                    let findat : TokenData;
                    unsafe {
                        findat = (*dat).clone();
                        Box::from_raw(dat);
                    }
                    self.scopes.set(n, Token::new(DAT, findat));
                },
                _ => {}
            }
        }
        let mut argtoks : Vec<Token> = match call.value {TokenData::List(l)=>l,_=>vec![]};
        let mut argbuild : Vec<Token> = Vec::new();
        let mut named : bool = false;
        let mut argpos : usize = 0;
        loop {
            if argtoks.len() == 0 {
                break;
            }
            let token = argtoks.remove(0);
            if token.id == SYM && token.value.string_ref() == "," {
                // named parameter
                if argbuild[1].id == OPS && argbuild[1].value.string_ref() == "=" {
                    named = true;
                } else if named {
                    return Err("positional arg after named arg".to_owned());
                }
                if named {
                    let v = self.eval_exp(argbuild[2..].to_vec())?;
                    self.scopes.set(argbuild[0].value.string_ref(), v);
                } else {
                    let v = self.eval_exp(argbuild)?;
                    self.scopes.set(fun.0[argpos].pair_ref().0, v);
                    argpos += 1;
                }
                argbuild = Vec::new();
            } else {
                argbuild.push(token);
            }
        }
        let r : Token = self.eval(fun.2)?;
        return match r.value.match_type(&fun.1) {true=>Ok(r),_=>Err("type mismatch in function return".to_owned())};
    }
    pub fn eval_exp (&mut self, mut tokens : Vec<Token>) -> EvalResultO {
        let mut i : usize = 0;
        let mut f : Token = tokens.remove(0);
        loop {
            if i >= tokens.len() {
                break;
            }
            if tokens[i].id == PRN {
                let x = self.eval_exp(match tokens.remove(i).value {TokenData::List(l)=>l,_=>vec![]})?;
                tokens.insert(i, x);
            } else if tokens[i].id == SYM {
                //
            } else if tokens[i].id == OPS {
                let mut tmp : Vec<Token> = Vec::new();
                let j : usize = i + 1;
                loop {
                    if j >= tokens.len() || tokens[j].id == OPS {
                        break;
                    }
                    tmp.push(tokens.remove(j));
                }
                let x = self.eval_exp(tmp)?;
                // tokens.insert(j, self.eval_exp(tmp)?);
                f = self.do_op(f, tokens.remove(i), x)?;
            }
            i += 1;
        }
        return Ok(f);
    }
    pub fn eval (&mut self, mut tokens : Vec<Token>) -> EvalResultO {
        let mut i : usize = 0;
        let mut l : usize = tokens.len();
        let mut build_flag : Flag = Flag::new();
        build_flag.set(Flags::Uninit, true);
        let mut keep_flag : bool = false;
        loop {
            if i >= l {
                break;
            }
            // println!("routine build flag bits: {}", build_flag.bits());
            let token : &Token = &tokens[i];
            // println!("TOKEN: {}", token);
            if token.id == OPCALL {
                let dumbstr : Option<&String> = None;
                let dumbvec : Option<&Vec<String>> = None;
                let components = &match &token.value {TokenData::OPCall(x,y)=>(x,y),_=>(dumbstr.unwrap(),dumbvec.unwrap())};
                self.binds.operations.execute(&components.0, &components.1, &mut self.scopes);
            } else if token.id == KEY {
                match match &token.value {TokenData::String(s)=>&s,_=>""} {
                    "return" => {
                        let mut fexp : Vec<Token> = Vec::new();
                        let j : usize = i + 1;
                        loop {
                            if j >= tokens.len() {
                                return Err("unclosed return statement".to_owned());
                            }
                            if tokens[j].id == NLN {
                                break;
                            }
                            fexp.push(tokens.remove(j));
                        }
                        return self.eval_exp(fexp);
                    },
                    "readonly" => {
                        build_flag.set(Flags::Readonly, true);
                        keep_flag = true;
                    },
                    "global" => {
                        build_flag.set(Flags::Global, true);
                        keep_flag = true;
                    },
                    "local" => {
                        build_flag.set(Flags::Global, false);
                        keep_flag = true;
                    },
                    "var" => {
                        let name = match &tokens[i+1].value {TokenData::Name(n)=>n.clone(),_=>{return Err("invalid var assignment".to_owned());}};
                        // println!("var create build flag bits: {}", build_flag.bits());
                        self.scopes.insert(&name, build_flag);
                    },
                    _ => {},
                };
            } else if token.id == OPS {
                match match &token.value {TokenData::String(s)=>&s,_=>""} {
                    "=" => {
                        let name = match &tokens[i-1].value {TokenData::Name(n)=>n.clone(),_=>{return Err("invalid var assignment".to_owned());}};
                        let mut lst : Vec<Token> = Vec::new();
                        let j : usize = i + 1;
                        if j >= l {
                            return Err("unfinished assignment".to_owned());
                        }
                        loop {
                            if j >= l || tokens[j].id == NLN {
                                break;
                            }
                            lst.push(tokens.remove(j));
                        }
                        if lst.len() == 0 {
                            return Err("no assigned value".to_owned());
                        }
                        let val = self.eval_exp(lst)?;
                        self.scopes.set(&name, val);
                        l = tokens.len();
                        // println!("{:?}", self.scopes);
                    },
                    _ => {},
                };
            }
            if !keep_flag {
                if build_flag.value != 128 {
                    build_flag = Flag::new();
                    build_flag.set(Flags::Uninit, true);
                }
            } else {
                keep_flag = false;
            }
            i += 1;
        }
        return Ok(void_token());
    }
    fn sysobj (&mut self, obj : Token) {
        let objname : &str = &(match &obj.value {TokenData::Class(s,_,_)=>s.clone(),_=>String::new()});
        let ptrname : String = self.scopes.ptr_alloc(&(String::from("sysobj-")+objname));
        self.scopes.heap.insert(ptrname.clone(), obj);
        self.scopes.insert(objname, Flag::new());
        self.scopes.set(objname, Token::new(PTR, TokenData::String(ptrname.clone())));
        self.scopes.flag(objname, Flags::System, true);
        self.scopes.flag(objname, Flags::Protected, true);
    }
    fn sysfunc (&mut self, name : &str, fun : Token) {
        let ptrname : String = self.scopes.ptr_alloc(&(String::from("syfun-")+name));
        self.scopes.heap.insert(ptrname.clone(), fun);
        self.scopes.set(name, Token::new(PTR, TokenData::String(ptrname.clone())));
        self.scopes.flag(name, Flags::Global, true);
        self.scopes.flag(name, Flags::System, true);
        self.scopes.flag(name, Flags::Protected, true);
    }
    fn init (&mut self) {
        let mut conh1 : HashMap<String, Token> = HashMap::new();
        // let mut conh2 : HashMap<String, Token> = HashMap::new();
        let f_conlog = Token::new(DAT, TokenData::Function(vec![TokenData::Pair("items".to_owned(), "list<any>".to_owned(), Box::new(TokenData::List(Vec::new())))], "void".to_owned(), vec![Token::new(OPCALL, TokenData::OPCall("console.log".to_owned(), vec!["items".to_owned()]))]));
        conh1.insert("log".to_owned(), f_conlog.clone());
        self.sysobj(Token::new(OBJ, TokenData::Class("console".to_owned(), conh1, HashMap::new())));
        self.sysfunc("clog", f_conlog);
        self.scopes.new_scope();
        // self.scopes.dump();
        println!("OVER HERE");
        match self.eval_func(self.scopes.get("clog"), Token::new(PRN, TokenData::List(vec![Token::new(DAT,TokenData::List(vec![Token::new(DAT,TokenData::String("test".to_owned()))]))]))) {Ok(t)=>{println!("{:?}",t);},Err(e)=>{println!("{}",e);}};
    }
    pub fn run (&mut self, tokens : Vec<Token>) -> Result<String, String> {
        self.init();
        printtoks(&tokens);
        match self.eval(tokens) {
            Ok(_) => {},
            Err(e) => {return Err(e);},
        };
        self.scopes.ptr_alloc("testptr");
        self.scopes.dump();
        // println!("{:?}", self.scopes);
        // self.scopes.set("test", Token::new(DAT, TokenData::String(String::from("xyz"))));
        // println!("{:?}", self.scopes.get(String::from("test")));
        // self.scopes.flag("test", Flags::System, true);
        // println!("{}", self.scopes.get_flag("test").bits());
        return Ok(String::from("placeholder"));
    }
}