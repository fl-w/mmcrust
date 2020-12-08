use std::collections::HashMap;

use super::{EvalError, EvalResult, Object};

pub type EnvScope = usize;

#[derive(Debug, PartialEq, Default)]
pub struct Frame {
    extend_scope: EnvScope,
    bindings: HashMap<String, Object>,
}

impl Frame {
    fn global() -> Self { Self::default() }
}

#[derive(Debug, PartialEq)]
pub struct Env {
    frames: Vec<Frame>,
    // if next
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
            extend_scope,
            bindings: HashMap::new(),
        });
    }

    pub fn current_scope(&self) -> EnvScope { self.frames.len() }

    pub fn declare_next(&mut self) { self.declare_next = true; }

    pub fn set(&mut self, key: String, value: Object) -> EvalResult {
        if self.declare_next {
            self.declare_next = false;
            Ok(self
                .frames
                .last_mut()
                .unwrap()
                .bindings
                .insert(key, value)
                .unwrap_or(Object::Void))
        } else if let Some(var) = self.get_mut(&key) {
            let prev = var.clone();
            *var = value;
            Ok(prev)
        } else {
            Err(EvalError::UnboundVariable(key.to_owned()))
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> { self.get_in_scope(key, self.current_scope()) }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Object> {
        self.get_mut_in_scope(key, self.current_scope())
    }

    pub fn get_mut_in_scope<'a>(
        &'a mut self,
        key: &str,
        scope: EnvScope,
    ) -> Option<&'a mut Object> {
        let acm = 0;
        self.frames.iter().nth
        if let Some(ref mut frame) = self.frames.get_mut(scope) {
            let v = frame.bindings.get_mut(key);
            if v.is_some() {
                v
            } else {
                if frame.extend_scope == scope {
                    None
                } else {
                    self.get_mut_in_scope(key, frame.extend_scope)
                }
            }
        } else {
            None
        }
    }

    pub fn get_in_scope(&self, key: &str, scope: EnvScope) -> Option<&Object> {
        self.frames.get(scope).and_then(|frame| {
            frame.bindings.get(key).or_else(|| {
                if frame.extend_scope == scope {
                    None
                } else {
                    self.get_in_scope(key, frame.extend_scope)
                }
            })
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
