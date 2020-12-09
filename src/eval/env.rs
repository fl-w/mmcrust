use std::collections::HashMap;

use super::{EvalError, EvalResult, Object};

pub type EnvScope = usize;

pub const GLOBAL_SCOPE: EnvScope = 0;

#[derive(Debug, PartialEq, Default)]
pub struct Frame {
    extends: EnvScope,
    bindings: HashMap<String, Object>,
}

impl Frame {
    fn global() -> Self { Self::default() }
}

#[derive(Debug, PartialEq)]
pub struct Env {
    frames: Vec<Frame>,
    declare_next: bool,
}

impl Env {
    pub fn new() -> Self {
        Self {
            frames: vec![Frame::global()],
            declare_next: false,
        }
    }

    pub fn drop_frame(&mut self) -> Option<Frame> {
        if self.current_scope() != 0 {
            self.frames.pop()
        } else {
            panic!("cannot drop global scope")
        }
    }

    pub fn extend(&mut self, extend_scope: EnvScope) {
        self.frames.push(Frame {
            extends: extend_scope,
            bindings: HashMap::new(),
        });
    }

    pub fn current_scope(&self) -> EnvScope { self.frames.len() }

    pub fn declare_next(&mut self) { self.declare_next = true; }

    pub fn declare(&mut self, key: String, value: Object) {
        self.frames.last_mut().unwrap().bindings.insert(key, value);
    }

    pub fn set(&mut self, key: String, value: Object) -> EvalResult {
        if self.declare_next {
            self.declare_next = false;
            self.declare(key, value);

            Ok(Object::Void)
        } else if let Some(var) = self.get_mut(&key) {
            let prev = var.clone();
            *var = value;
            Ok(prev)
        } else {
            Err(EvalError::UnboundVariable(key.to_owned()))
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> { self.get_in_scope(key, self.current_scope()) }

    pub fn get_in_scope(&self, key: &str, scope: EnvScope) -> Option<&Object> {
        self.find_from_scope(key, scope)
            .map(|scope| self.frames[scope].bindings.get(key).unwrap())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Object> {
        self.get_mut_in_scope(key, self.current_scope())
    }

    pub fn get_mut_in_scope(&mut self, key: &str, scope: EnvScope) -> Option<&mut Object> {
        if let Some(scope) = self.find_from_scope(key, scope) {
            self.frames[scope].bindings.get_mut(key)
        } else {
            None
        }
    }

    fn find_from_scope(&self, key: &str, scope: EnvScope) -> Option<EnvScope> {
        self.frames.get(scope).and_then(|frame: &Frame| {
            if frame.bindings.contains_key(key) {
                Some(scope)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod env_tests {
    use super::*;

    #[test]
    fn test_env_get() {
        let mut env = Env::new();

        env.set("int_0".to_owned(), Object::Int(0));
        env.set("int_1".to_owned(), Object::Int(0));
        env.set("int_2".to_owned(), Object::Int(0));

        assert_eq!(env.get(&"int_0"), Some(&Object::Int(0)));
    }

    #[test]
    fn test_env_set() {
        let mut env = Env::new();
        let mut map: HashMap<String, Object> = HashMap::new();

        env.set("int_0".to_owned(), Object::Int(0));
        map.insert("int_0".to_owned(), Object::Int(0));

        // assert_eq!(env.frames, vec![map]);
    }
}
