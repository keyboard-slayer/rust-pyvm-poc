use cpython::{PyObject, Python};
use std::collections::VecDeque;
use std::process::exit;

#[derive(Debug)]
pub struct PyStack {
    stack: VecDeque<PyObject>,
}

impl PyStack {
    pub fn new(_py: Python) -> PyStack {
        PyStack {
            stack: VecDeque::new(),
        }
    }

    pub fn object_check(&mut self, _obj: &PyObject) -> bool {
        true
    }

    pub fn unsafe_push_front(&mut self, obj: PyObject) {
        self.stack.push_front(obj);
    }

    pub fn unsafe_push_back(&mut self, obj: PyObject) {
        self.stack.push_back(obj);
    }

    pub fn safe_push_back(&mut self, obj: PyObject) {
        if self.object_check(&obj) {
            self.unsafe_push_back(obj)
        } else {
            eprintln!("UNSAFE !!!");
            exit(1);
            // TODO RAISE EXCEPTION
        }
    }

    pub fn safe_push_front(&mut self, obj: PyObject) {
        if self.object_check(&obj) {
            self.unsafe_push_front(obj)
        } else {
            eprintln!("UNSAFE !!!");
            exit(1);
        }
    }

    pub fn unsafe_pop(&mut self) -> Option<PyObject> {
        self.stack.pop_back()
    }

    pub fn safe_pop(&mut self) -> Option<PyObject> {
        if let Some(obj) = self.unsafe_pop() {
            if self.object_check(&obj) {
                return Some(obj);
            } else {
                eprintln!("UNSAFE !!!");
                exit(1);
            }
        } else {
            None
        }
    }
}
