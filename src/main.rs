mod instr;
mod vm;
#[macro_use]
use std::env;
use std::fs;
use instr::Inst::*;
use instr::Value::*;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(filename) => filename,
        None => panic!("Needs filename"),
    };
    let src = fs::read_to_string(filename).expect("Read Error");
    let p: instr::Program = match serde_sexpr::from_str(&src) {
        Ok(p) => p,
        _ => { println!("Parse error!"); return; }
    };
    if !p.valid() { panic!("Invalid Bytecode"); }
    vm::VM::new(p).run_program();
}
