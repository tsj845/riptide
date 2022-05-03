// constant definitions
#![allow(dead_code)]

pub const ERROR_RED : &str = "\x1b[38;2;250;100;50m";
pub const WARN_ORANGE : &str = "\x1b[38;2;200;175;0m";
pub const GOOD_GREEN : &str = "\x1b[38;2;0;215;50m";
pub const TEXT_NORMAL : &str = "\x1b[0m";

pub const KEY : u8 = 0; // keyword
pub const CTL : u8 = 1; // control flow
pub const DAT : u8 = 2; // data
pub const REF : u8 = 3; // reference
pub const PTR : u8 = 4; // pointer
pub const OPS : u8 = 5; // operator
pub const SYM : u8 = 6; // symbol
pub const INV : u8 = 7; // invalid
pub const NLN : u8 = 8; // line seperator
pub const TYP : u8 = 9; // type
pub const GRP : u8 = 10; // groupings
pub const PRN : u8 = 11; // parentheses groupings eg. (8 - 2) * 6 or (i - 2).toString()
pub const USB : u8 = 12; // uninitialized data contained in square brackets
pub const UCB : u8 = 13; // uninitialized data contained in curly brackets
pub const OBJ : u8 = 14; // object

pub const OPCALL : u8 = 255; // call to backend rust operation

pub const IDMAP : [&str; 16] = ["NODISPLAY", "KEY", "CTL", "DAT", "REF", "PTR", "OPS", "SYM", "INV", "NLN", "TYP", "GRP", "PRN", "USB", "UCB", "OBJ"];

// regex patterns
pub const NUMBER_RE_PAT : &str = r"^(0b[01]+|0x[0-9a-f]+|[0-9]+(\.[0-9]+)?)";
pub const DECI_RE_PAT : &str = r"^[0-9]+$";
pub const OP_RE_PAT : &str = r"^([<>|&+*=-]{1,2}|([+*/%!-]=?)|\$)$";
pub const KEY_RE_PAT : &str = r"\b(func|return|halt|raise|try|catch|final|if|else|for|while|in|class|static|private|readonly|self|var|import|from|as|enum|undefined|global|local)\b";
// pub const SYM_RE_PAT : &str = r"\b(\.|:|->)\b";
// pub const TYPE_RE_PAT : &str = r"\b(int|str|uint|float|number|bool|void|list|dict|any)\b";

// pub fn test () {
//     println!("{}error red {} warning orange {} good green {} normal text", ERROR_RED, WARN_ORANGE, GOOD_GREEN, TEXT_NORMAL);
// }

// // enums for altids
// #[derive(Clone, Copy)]
// pub enum AltId {
//     GrpCur0, // opening curly bracket
//     GrpCur1, // closing curly bracket
//     GrpSqr0, // opening square bracket
//     GrpSqr1, // closing square bracket
//     GrpPrn0, // opening parentheses
//     GrpPrn1, // closing parentheses
//     CtlIf0, // if
//     CtlIf1, // else
//     CtlLoop0, // for (range)
//     CtlLoop1, // for (in)
//     CtlLoop2, // while
//     KeyFunc, // keyword func
//     KeyReturn, // keyword return
//     KeyHalt, // fatal
//     KeyRaise, // raise error
//     KeyTry, // try
//     KeyCatch, // catch
//     KeyFinal, // final
//     KeyIf, // if
//     KeyElse, // else
//     KeyFor, // for loops
//     KeyWhile, // while loops
//     KeyIn, // comparason / for .. in loops
//     KeyClass, // class declaration
//     KeyStatic, // static members
//     KeyPrivate, // private data
//     KeyReadonly, // readonly data
//     KeySelf, // instance reference
//     KeyVar, // declare variable
//     KeyImport, // import module
//     KeyFrom, // selective import
//     KeyAs, // type cast / alias
//     KeyEnum, // enum declaration
//     KeyUndefined, // undefined
//     OpsPlus, // plus / concat
//     OpsMinus, // minus
//     OpsMul, // multiplication / repetition
//     OpsDiv, // division
//     OpsMod, // modulo
//     OpsPow, // exponentiation
//     OpsAss, // assignment
//     OpsPle, // plus equals
//     OpsMie, // minus equals
//     OpsMue, // times equals
//     OpsDve, // divide equals
//     OpsMde, // mod equals
//     OpsEql, // equal
//     OpsNeq, // not euqal
//     OpsNot, // logical not / bitwise inversion
//     OpsBnd, // bitwise and
//     OpsBor, // bitwise or
//     OpsLnd, // logical and
//     OpsLor, // logical or
//     OpsXor, // bitwise xor
//     OpsSim, // similar
//     OpsLtn, // less than
//     OpsGtn, // greater than
//     OpsLte, // less than or equal
//     OpsGte, // greater than or equal
//     Invalid, // invalid id
// }

// impl AltId {
//     pub fn decode (id : u8, altid : u8) -> AltId {
//         return AltId::Invalid;
//     }
// }

// pub enum GrpIds {
//     Open = 0,
//     Curly = 1,
//     Square = 2,
//     Paren = 3,
// }