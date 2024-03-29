use std::collections::HashMap;

use super::{EvalError, EvalResult, Object};

pub type EnvScope = usize;

pub const GLOBAL_SCOPE: EnvScope = 0;

#[derive(Debug, PartialEq, Default)]
pub struct Frame {
    extends: EnvScope,
    bindings: HashMap<String, Object>,
    declare_next: bool,
}

impl Frame {
    fn global() -> Self { Self::default() }
}

#[derive(Debug, PartialEq)]
pub struct Env {
    frames: Vec<Frame>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            frames: vec![Frame::global()],
        }
    }

    pub fn drop_frame(&mut self) -> Option<Frame> {
        if self.current_scope() != 0 {
            self.frames.pop()
        } else {
            panic!("cannot drop global scope")
        }
    }

    fn add_frame(&mut self, from_scope: EnvScope) {
        self.frames.push(Frame {
            extends: from_scope,
            ..Default::default()
        });
    }

    pub fn extend_scope<F, T>(&mut self, scope: EnvScope, closure: F) -> T
    where
        F: FnOnce(&mut Env) -> T,
    {
        self.add_frame(scope);

        let rs = closure(self);

        if !self.frames[self.current_scope()]
            .bindings
            .values()
            .any(|obj| matches!(*obj, Object::Closure(_, _)))
        {
            // drop frame if no closures were defined in the current frame
            self.drop_frame();
        }

        rs
    }

    pub fn current_scope(&self) -> EnvScope { self.frames.len() - 1 }

    fn frame_mut(&mut self) -> &mut Frame { self.frames.last_mut().unwrap() }

    fn frame(&self) -> &Frame { self.frames.last().unwrap() }

    pub fn declare_next(&mut self) { self.frame_mut().declare_next = true; }

    pub fn undeclare_next(&mut self) { self.frame_mut().declare_next = false; }

    pub fn declare(&mut self, key: String, value: Object) -> EvalResult {
        self.frame_mut()
            .bindings
            .insert(key.clone(), value)
            .map(|_| Err(EvalError::Redeclaration(key)))
            .unwrap_or(Ok(Object::Void))
    }

    pub fn set(&mut self, key: String, value: Object) -> EvalResult {
        if self.frame().declare_next {
            self.undeclare_next();
            self.declare(key, value)
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
        self.find_scope(key, scope)
            .map(|scope| self.frames[scope].bindings.get(key).unwrap())
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Object> {
        self.get_mut_in_scope(key, self.current_scope())
    }

    pub fn get_mut_in_scope(&mut self, key: &str, scope: EnvScope) -> Option<&mut Object> {
        if let Some(scope) = self.find_scope(key, scope) {
            self.frames[scope].bindings.get_mut(key)
        } else {
            None
        }
    }

    fn find_scope(&self, key: &str, scope: EnvScope) -> Option<EnvScope> {
        let frame = &self.frames[scope];

        if frame.bindings.contains_key(key) {
            Some(scope)
        } else if frame.extends == scope {
            None // circular dependency found
        } else {
            self.find_scope(key, frame.extends)
        }
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
