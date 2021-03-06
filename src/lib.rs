#![allow(non_snake_case)]
#[macro_use]
extern crate cpython;

mod opcode;
mod stack;

use cpython::{exc, PyErr, Python, PyObject, PyDict, PyClone, PyResult, PythonObject, ObjectProtocol, PyTuple, PyModule, NoArgs};
use std::u8;

fn to_byte_vec(hex: String) -> Vec<u8> 
{
    let mut bytecode: Vec<u8> = Vec::new();
    for i in (0..hex.len()).step_by(2) 
    {
        let byte = format!("{}{}", hex.chars().nth(i).unwrap(), hex.chars().nth(i+1).unwrap());
        bytecode.push(u8::from_str_radix(&byte, 16).unwrap());
    }

    bytecode
}

fn reval(py: Python, bytecode: PyObject, globals: Option<PyDict>, locals: Option<PyDict>) -> PyResult<PyObject> 
{
    let local_var;
    let global_var;


    if let None = globals 
    {
        global_var = py.eval("globals()", None, None).unwrap().cast_as::<PyDict>(py).unwrap().copy(py).unwrap();
    } 
    else 
    {
        global_var = globals.unwrap();
    }

    if let None = locals 
    {
        local_var = global_var.copy(py).unwrap();
    } 
    else 
    {
        local_var = locals.unwrap();
    }

    println!("=== {} === ", bytecode.getattr(py, "co_name").unwrap());
    let scope = global_var.copy(py).unwrap();
    println!("Globals: {:?}", global_var.items(py));
    println!("Locals: {:?}", local_var.items(py));
    _ = scope.set_item(py, "func", &bytecode);
    _ = scope.set_item(py, "dis", PyModule::import(py, "dis").unwrap());
    _ = scope.set_item(py, "sys", PyModule::import(py, "sys").unwrap());
    _ = py.eval("sys.stdout.write(dis.code_info(func))", Some(&scope), Some(&local_var));
    println!("\n");
    _ = py.eval("dis.dis(func)", Some(&scope), Some(&local_var));

    let code = bytecode.getattr(py, "co_code").unwrap();
    let names = bytecode.getattr(py, "co_names").unwrap();
    let consts = bytecode.getattr(py, "co_consts").unwrap();
    let varnames = bytecode.getattr(py, "co_varnames").unwrap();

    let builtins_module = global_var.get_item(py, "__builtins__").unwrap();
    let builtins_dict;

    if let Ok(builtins_dict_obj) = builtins_module.getattr(py, "__dict__") 
    {
        if let Ok(builtins_dict_res) = builtins_dict_obj.cast_as::<PyDict>(py).unwrap().copy(py) 
        {
            builtins_dict = builtins_dict_res;
        }
        else 
        {
            return Err(PyErr::new::<exc::RuntimeError, _>(py, "Couldn't copy globals() values"));
        }
    } 
    else if let Ok(builtins_dict_res) = builtins_module.cast_as::<PyDict>(py) 
    {
        builtins_dict = builtins_dict_res.copy(py).unwrap();
    }
    else 
    {
        return Err(PyErr::new::<exc::SyntaxError, _>(py, "A dictionnary or a module as intended for globals"));
    }

    let hex = code.call_method::<NoArgs>(py, "hex", NoArgs, None);
    let mut opcode = to_byte_vec(hex.unwrap().to_string()).into_iter();
    let mut stack: stack::PyStack = stack::PyStack::new(py);

    while let Some(byte) = opcode.next() 
    {
        match byte 
        {
            opcode::POP_TOP => {
                _ = opcode.nth(0); // We ignore the next argument
                _ = stack.unsafe_pop();
            }
            opcode::RETURN_VALUE => {
                _ = opcode.nth(0);

                if let Some(value) = stack.safe_pop() {
                    return Ok(value);
                } else {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::LOAD_CONST => {
                if let Some(consti) = opcode.nth(0) {
                    if let Ok(const_val) = consts.get_item(py, consti) {
                        stack.safe_push_back(const_val);
                    } else {
                        return Err(PyErr::new::<exc::RuntimeError, _>(py, "Contant out of range"));
                    }
                } else {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::LOAD_NAME => {
                if let Some(namei) = opcode.nth(0) {
                    let varname = names.get_item(py, namei).unwrap();

                    if let Some(varcontent) = local_var.get_item(py, &varname) 
                    {
                        stack.safe_push_back(varcontent);
                    } 
                    else if let Some(varcontent) = local_var.get_item(py, &varname) 
                    {
                        stack.safe_push_back(varcontent);
                    } 
                    else 
                    {
                        return Err(PyErr::new::<exc::NameError, _>(py, format!("name {} is not defined", varname)));
                    }
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::BUILD_MAP => {
                if let Some(count) = opcode.nth(0) 
                {
                    let dict = PyDict::new(py);

                    for _ in 0..count 
                    {
                        let value = stack.safe_pop();
                        _ = dict.set_item(py, stack.safe_pop(), value);
                    }

                    stack.safe_push_back(dict.into_object());
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::LOAD_ATTR => {
                if let Some(namei) = opcode.nth(0) 
                {
                    if let Some(tos) = stack.safe_pop()
                    {
                        if let Ok(attrname) = names.get_item(py, namei) 
                        {
                            if let Ok(attr) = tos.getattr(py, &attrname) 
                            {
                                stack.safe_push_front(attr);
                            } 
                            else 
                            {
                                return Err(PyErr::new::<exc::AttributeError, _>(py, format!("type object {} has no attribute '{}'", tos, attrname)));
                            }
                        } 
                        else 
                        {
                            return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing constant"));
                        }
                    } 
                    else 
                    {
                        return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing object in TOS"));
                    }
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::LOAD_GLOBAL => {
                if let Some(namei) = opcode.nth(0)
                {
                    if let Ok(name) = names.get_item(py, namei) 
                    {
                        stack.safe_push_back(name);
                    } 
                    else 
                    {
                        return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing global variable"));
                    }
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::LOAD_FAST => {
                if let Some(var_num) = opcode.nth(0) 
                {
                    if let Ok(varname) = varnames.get_item(py, var_num) 
                    {
                        if let Some(var) = local_var.get_item(py, &varname) 
                        {
                            stack.safe_push_back(var);
                        } 
                        else 
                        {
                            return Err(PyErr::new::<exc::NameError, _>(py, format!("name {} is not defined", varname)));
                        }
                    } 
                    else 
                    {
                        return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing variable"));
                    }
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::STORE_FAST => {
                if let Some(var_num) = opcode.nth(0) 
                {
                    let tos = stack.safe_pop().unwrap();
                    let varname = varnames.get_item(py, var_num).unwrap();
                    _ = local_var.set_item::<String, PyObject>(py, varname.to_string(), tos);
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            opcode::CALL_FUNCTION => {
                if let Some(argc) = opcode.nth(0) 
                {
                    let mut args: Vec<PyObject> = Vec::new();

                    for i in 0..argc 
                    {
                        if let Some(arg) = stack.safe_pop() 
                        {
                            args.push(arg);
                        } 
                        else 
                        {
                            return Err(PyErr::new::<exc::TypeError, _>(py, format!("Missing {} argument(s)", i-argc)));
                        }
                    }

                    args.reverse();

                    if let Some(func) = stack.unsafe_pop() 
                    {
                        if let Some(funcobj) = builtins_dict.get_item(py, &func)
                        {
                            if argc == 0 
                            {
                                match funcobj.call::<NoArgs>(py, NoArgs, None)
                                {
                                    Ok(val) => stack.safe_push_back(val),
                                    Err(e) => return Err(e)
                                }
                            }
                            else 
                            {
                                let tuple_arg = PyTuple::new(py, args.as_slice());
                                match funcobj.call(py, tuple_arg, None) 
                                {
                                    Ok(val) => stack.safe_push_back(val),
                                    Err(e) => return Err(e)
                                }
                            }
                            
                        }
                        else if global_var.contains(py, &func).unwrap()
                        {
                            let scope = global_var.copy(py).unwrap();
                            let code_obj = global_var.get_item(py, &func).unwrap();
                            let func = code_obj.getattr(py, "__code__").unwrap();
                            let argc_func = func.getattr(py, "co_argcount").unwrap().extract::<usize>(py).unwrap();
                            let varnames_func = func.getattr(py, "co_varnames").unwrap();

                            for index in 0..argc_func 
                            {
                                let var = varnames_func.iter(py).unwrap().nth(index).unwrap().unwrap();
                                let value = args[index].clone_ref(py);
                                _ = scope.set_item::<String, PyObject>(py, var.to_string(), value);
                            }
                            stack.safe_push_back(reval(py, func, Some(global_var.copy(py).unwrap()), Some(scope)).unwrap());
                        } 
                        else if func.is_callable(py) 
                        {
                            if func.get_type(py).name(py) == "method" 
                            {
                                println!("OK");
                                args.reverse();
                                args.push(func.getattr(py, "__self__").unwrap());
                                args.reverse();
                            }

                            let code = func.getattr(py, "__code__").unwrap();
                            if argc == 0 
                            {
                                match reval(py, code, Some(global_var.copy(py).unwrap()), None)
                                {
                                    Ok(ret) => stack.safe_push_back(ret),
                                    Err(e) => return Err(e)
                                }
                            } 
                            else 
                            {
                                let argcount = code.getattr(py, "co_argcount").unwrap().extract::<usize>(py).unwrap();
                                let argnames = &code.getattr(py, "co_varnames").unwrap().extract::<Vec<String>>(py).unwrap();

                                if let Ok(funcscope) = global_var.copy(py)
                                {
                                    for i in 0..argcount 
                                    {
                                        if let Some(argval) = args.get(i) 
                                        {
                                            _ = funcscope.set_item::<&str, PyObject>(py, argnames.get(i).unwrap().as_str(), argval.clone_ref(py));
                                        }
                                    }

                                    match reval(py, code, Some(global_var.copy(py).unwrap()), Some(funcscope)) 
                                    {
                                        Ok(ret) => stack.safe_push_back(ret),
                                        Err(e) => return Err(e)
                                    }
                                }
                                else 
                                {
                                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "Couldn't create new local scope"));
                                }
                            }
                        } 
                        else 
                        {
                            return Err(PyErr::new::<exc::NameError, _>(py, format!("function {} is not defined", func)));
                        }
                    } 
                    else 
                    {
                        return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                    }
                } 
                else 
                {
                    return Err(PyErr::new::<exc::RuntimeError, _>(py, "invalid bytecode, missing argument"));
                }
            }
            _ => {
                return Err(PyErr::new::<exc::NotImplementedError, _>(py, format!("bytecode {} not implemented", byte)))
            }
        }
    }

    unreachable!();
}

py_module_initializer!(libpyreval, initlibpyreval, PyInit_status, |py, m| { 
    m.add(py, "__doc__", "This module is implemented in Rust.")?; 
    m.add(py, "reval", py_fn!(py, reval(bytecode: PyObject, globals: Option<PyDict>, locals: Option<PyDict>)))?; 
    Ok(()) 
});