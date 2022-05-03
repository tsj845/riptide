use crate::statics::*;
use crate::tokens::*;
use std::collections::HashMap;
use rand::random;

#[allow(dead_code)]
pub enum Flags {
    Global = 0, // in global scope
    Readonly = 1, // readonly
    Private = 2, // private
    Exposed = 3, // exposed from library
    Uninit = 4, // uninitialized data
    Protected = 5, // managed by system
    System = 6, // system object
    Active = 7, // this flag is valid
}

#[derive(Clone, Copy)]
pub struct Flag {
    pub value : u8,
}

impl std::fmt::Debug for Flag {
    fn fmt (&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Flag")
        .field("value", &self.bits())
        .finish()
    }
}

impl Flag {
    pub fn null () -> Flag {
        Flag {
            value : 0,
        }
    }
    pub fn new () -> Flag {
        Flag {
            value : 0b1u8 << 7,
        }
    }
    pub fn bits (&self) -> String {
        let mut f : String = String::new();
        for i in [0,1,2,3,4,5,6,7] {
            f.push(char::from_digit(((self.value & 1u8 << i as u8) >> i as u8) as u32, 10).unwrap());
        }
        unsafe {
            f.as_bytes_mut().reverse();
        }
        return f;
    }
    pub fn or (self, other : Flag) -> Flag {
        if self.valid() {
            return self;
        }
        return other;
    }
    pub fn set (&mut self, target : Flags, value : bool) {
        match value {
            false => {self.value = self.value & !(1u8 << target as u8)},
            true => {self.value = self.value | (1u8 << target as u8)}
        }
    }
    pub fn valid (&self) -> bool {
        return self.value & (1u8 << 7) == 128;
    }
    pub fn matches (&self, target : Flags, value : bool) -> bool {
        return self.valid() && (self.value & (1u8 << target as u8) > 0) == value;
    }
    // pub fn matching (&self, targets : Vec<Flags>, values : Vec<bool>) -> bool {
    //     if !self.valid() {
    //         return false;
    //     }
    //     for (target, value) in targets.into_iter().zip(values.into_iter()) {
    //         if !self.matches(target, value) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
}

// handles scopes
#[derive(Debug)]
pub struct ScopeManager {
    // complex objects
    pub heap : HashMap<String, Token>,
    // normal scopes
    pub scopes : Vec<HashMap<String, (Token, Flag)>>,
    // pointer allocations
    p_allocs : Vec<Vec<String>>,
    // quick accessor
    scope_count : usize,
}

impl ScopeManager {
    pub fn new () -> ScopeManager {
        ScopeManager {
            heap : HashMap::new(),
            scopes : vec![HashMap::new()],
            p_allocs : vec![vec![]],
            scope_count : 1,
        }
    }
    pub fn dump (&self) {
        println!("\n");
        // dump scoped vars and ptrs
        println!("SCOPES");
        for i in [0, self.scope_count-1] {
            if i >= self.scope_count {
                break;
            }
            println!("{} scope:", ["global", "local"][match i>0{true=>1,_=>0}]);
            for (k, (t, f)) in &self.scopes[i] {
                println!("{} : ({}, {})", k, t, f.bits());
            }
            println!("\n{:?}\n\n", self.p_allocs[i]);
        }
        // dump heap
        println!("HEAP");
        for (k, v) in &self.heap {
            println!("{} : {:?}", k, v);
        }
        println!("\n");
    }
    pub fn new_scope (&mut self) {
		self.scope_count += 1;
		self.scopes.push(HashMap::new());
        self.p_allocs.push(Vec::new());
	}
	pub fn rem_scope (&mut self) {
		self.scope_count -= 1;
        self.scopes.pop();
		for key in &self.p_allocs[self.scope_count] {
			self.heap.remove(&key[..]);
		}
		self.p_allocs.pop();
	}
    // ensures that all pointers are unique
	fn get_ptr_name (&self, name : &str) -> String {
		let mut cand = String::from("-scope-") + &(self.scope_count-1).to_string() + name + &random::<u32>().to_string();
		loop {
			if !self.heap.contains_key(&cand) {
				break;
			}
			cand = String::from("-scope-") + &(self.scope_count-1).to_string() + name + &random::<u32>().to_string();
		}
		return cand;
	}
	pub fn ptr_alloc (&mut self, ptr_name : &str) -> String {
        let name = self.get_ptr_name(ptr_name);
        self.heap.insert(name.clone(), void_token());
		self.p_allocs[self.scope_count-1].push(name.clone());
        return name;
	}
    pub fn get_flag (&self, name : &str) -> Flag {
        if self.scopes[self.scope_count-1].contains_key(name) {
            return self.scopes[self.scope_count-1].get(name).unwrap().1;
        }
        if self.scopes[0].contains_key(name) {
            return self.scopes[0].get(name).unwrap().1;
        }
        return Flag::null();
    }
    fn get_from (&self, scope : usize, name : &str) -> Token {
        if self.scopes[scope].contains_key(name) {
            return self.scopes[scope].get(name).unwrap().0.clone();
        }
        return void_token();
    }
    pub fn flag (&mut self, name : &str, target : Flags, value : bool) {
        let mut f : Flag = self.get_flag(name);
        // println!("FB: {}, {}", f.bits(), f.valid());
        if f.valid() {
            let log : bool = f.matches(Flags::Global, true);
            f.set(target, value);
            let v = self.scopes[match log {true => 0, _ => self.scope_count-1}].get_mut(name).unwrap();
            v.1 = f;
            // println!("{}, {}", f.bits(), v.1.bits());
        }
    }
    pub fn get (&self, name : &str) -> Token {
        if self.get_flag(name).matches(Flags::Global, true) {
            return self.get_from(0, name);
        }
        return self.get_from(self.scope_count-1, name);
    }
    pub fn set (&mut self, name : &str, value : Token) {
        let mut f : Flag = self.get_flag(name);
        if f.matches(Flags::System, true) || (f.matches(Flags::Readonly, true) && f.matches(Flags::Uninit, false)) {
            return;
        }
        f.set(Flags::Uninit, false);
        if f.matches(Flags::Global, true) {
            self.scopes[0].insert(name.to_owned(), (value, f));
            return;
        }
        self.scopes[self.scope_count-1].insert(name.to_owned(), (value, f.or(Flag::new())));
    }
    pub fn insert (&mut self, name : &str, flag : Flag) {
        if self.get_flag(name).matches(Flags::System, true) {
            return;
        }
        self.scopes[match flag.matches(Flags::Global, false) {true=>self.scope_count-1,_=>0}].insert(name.to_owned(), (void_token(), flag));
    }
    pub fn deref (&self, nameo : &str) -> Token {
        let mut name : String = nameo.to_owned();
        let mut checked : Vec<String> = Vec::new();
        let mut f : Token;
        loop {
            if checked.contains(&name) {
                return void_token();
            }
            checked.push(name.clone());
            f = self.get(&name);
            if f.id != REF {
                return f;
            }
            name = f.value.unwrap_string();
        }
    }
    pub fn deref_ptr (&self, ptr : Token) -> Token {
        if ptr.id != PTR {
            return ptr;
        }
        // println!("PTR: {}", ptr);
        match ptr.value {
            TokenData::String(s) => self.heap.get(&s).unwrap().clone(),
            _ => void_token()
        }
    }
    pub fn deref_full (&self, name : &str) -> Token {
        return self.deref_ptr(self.deref(name));
    }
}