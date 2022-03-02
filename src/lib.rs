#![allow(non_snake_case)]
#[macro_use]
extern crate cpython;

mod opcode;
mod stack;

use cpython::{Python, PyObject, PyResult, ObjectProtocol, NoArgs};
use std::process::exit;
use std::u8;

fn to_byte_vec(hex: String) -> Vec<u8> {
    let mut bytecode: Vec<u8> = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = format!("{}{}", hex.chars().nth(i).unwrap(), hex.chars().nth(i+1).unwrap());
        bytecode.push(u8::from_str_radix(&byte, 16).unwrap());
    }

    bytecode
}

fn reval(py: Python, bytecode: PyObject) -> PyResult<PyObject> {
    let code = bytecode.getattr(py, "co_code").unwrap();
    let names = bytecode.getattr(py, "co_names").unwrap();
    let consts = bytecode.getattr(py, "co_consts").unwrap();

    let hex = code.call_method::<NoArgs>(py, "hex", NoArgs, None);
    let mut opcode = to_byte_vec(hex.unwrap().to_string()).into_iter();
    let mut stack: stack::PyStack = stack::PyStack::new(py);

    while let Some(byte) = opcode.next() {
        match byte {
            opcode::POP_TOP => {
                _ = opcode.nth(0); // We ignore the next argument
                _ = stack.unsafe_pop();
            }
            opcode::RETURN_VALUE => {
                _ = opcode.nth(0);
                return Ok(stack.safe_pop().unwrap());
            }
            opcode::LOAD_CONST => {
                if let Some(consti) = opcode.nth(0) {
                    stack.safe_push(consts.get_item(py, consti).unwrap());
                } else {
                    panic!("");
                }
            }
            opcode::LOAD_GLOBAL => {
                if let Some(namei) = opcode.nth(0) {
                    stack.safe_push(names.get_item(py, namei).unwrap());
                } else {
                    panic!("");
                }
            }
            opcode::CALL_FUNCTION => {
                if let Some(argc) = opcode.nth(0) {
                    let mut args: Vec<PyObject> = Vec::new();

                    for _ in 0..argc {
                        if let Some(arg) = stack.safe_pop() {
                            args.push(arg);
                        } else {
                            panic!("Not enough argument !");
                        }
                    }

                    let args_repr = args.iter().map(|obj| obj.repr(py).unwrap()).collect::<Vec<_>>();
                    let args_repr_str = args_repr.iter().map(|obj| obj.to_string(py).unwrap()).collect::<Vec<_>>();

                    if let Some(func) = stack.unsafe_pop() {
                        _ = py.eval(format!("{}({})", func, args_repr_str.join(",")).as_str(), None, None);
                    } else {
                        panic!("Not enough argument !");
                    }
                } else {
                    panic!("");
                }
            }
            _ => {
                eprintln!("Uncoded bytecode {}", byte);
                exit(1);
            }
        }
    }

    unreachable!();
}

py_module_initializer!(libpyreval, initlibpyreval, PyInit_status, |py, m| { 
    m.add(py, "__doc__", "This module is implemented in Rust.")?; 
    m.add(py, "reval", py_fn!(py, reval(bytecode: PyObject)))?; 
    Ok(()) 
});