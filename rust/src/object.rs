use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::{BlockStatement, Identifier};

pub type BuiltinFunction = fn(&[Object]) -> Object;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Null,
    Error,
    Integer,
    Boolean,
    String,
    ReturnValue,
    Function,
    Builtin,
    Array,
    Hash,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectType::Null => write!(f, "NULL"),
            ObjectType::Error => write!(f, "ERROR"),
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::String => write!(f, "STRING"),
            ObjectType::ReturnValue => write!(f, "RETURN_VALUE"),
            ObjectType::Function => write!(f, "FUNCTION"),
            ObjectType::Builtin => write!(f, "BUILTIN"),
            ObjectType::Array => write!(f, "ARRAY"),
            ObjectType::Hash => write!(f, "HASH"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashKey {
    pub object_type: ObjectType,
    pub value: u64,
}

pub trait Hashable {
    fn hash_key(&self) -> HashKey;
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    ReturnValue(ReturnValue),
    Error(Error),
    Function(Function),
    String(StringObj),
    Builtin(Builtin),
    Array(Array),
    Hash(HashObj),
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null(_) => ObjectType::Null,
            Object::ReturnValue(_) => ObjectType::ReturnValue,
            Object::Error(_) => ObjectType::Error,
            Object::Function(_) => ObjectType::Function,
            Object::String(_) => ObjectType::String,
            Object::Builtin(_) => ObjectType::Builtin,
            Object::Array(_) => ObjectType::Array,
            Object::Hash(_) => ObjectType::Hash,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Null(n) => n.inspect(),
            Object::ReturnValue(rv) => rv.inspect(),
            Object::Error(e) => e.inspect(),
            Object::Function(f) => f.inspect(),
            Object::String(s) => s.inspect(),
            Object::Builtin(b) => b.inspect(),
            Object::Array(a) => a.inspect(),
            Object::Hash(h) => h.inspect(),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a.value == b.value,
            (Object::Boolean(a), Object::Boolean(b)) => a.value == b.value,
            (Object::Null(_), Object::Null(_)) => true,
            (Object::String(a), Object::String(b)) => a.value == b.value,
            _ => false,
        }
    }
}

impl Object {
    pub fn hash_key(&self) -> Result<HashKey, String> {
        match self {
            Object::Integer(i) => Ok(i.hash_key()),
            Object::Boolean(b) => Ok(b.hash_key()),
            Object::String(s) => Ok(s.hash_key()),
            _ => Err(format!("unusable as hash key: {}", self.object_type())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }

    pub fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl Hashable for Integer {
    fn hash_key(&self) -> HashKey {
        HashKey {
            object_type: ObjectType::Integer,
            value: self.value as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean { value }
    }

    pub fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl Hashable for Boolean {
    fn hash_key(&self) -> HashKey {
        let value = if self.value { 1 } else { 0 };
        HashKey {
            object_type: ObjectType::Boolean,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Null;

impl Null {
    pub fn new() -> Self {
        Null
    }

    pub fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Box<Object>,
}

impl ReturnValue {
    pub fn new(value: Object) -> Self {
        ReturnValue {
            value: Box::new(value),
        }
    }

    pub fn inspect(&self) -> String {
        self.value.inspect()
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }

    pub fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement, env: Rc<RefCell<Environment>>) -> Self {
        Function {
            parameters,
            body,
            env,
        }
    }

    pub fn inspect(&self) -> String {
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_string()).collect();
        format!("fn({}) {{\n{}\n}}", params.join(", "), self.body)
    }
}

#[derive(Debug, Clone)]
pub struct StringObj {
    pub value: String,
}

impl StringObj {
    pub fn new(value: String) -> Self {
        StringObj { value }
    }

    pub fn inspect(&self) -> String {
        self.value.clone()
    }
}

impl Hashable for StringObj {
    fn hash_key(&self) -> HashKey {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.value.hash(&mut hasher);
        HashKey {
            object_type: ObjectType::String,
            value: hasher.finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Builtin {
    pub function: BuiltinFunction,
}

impl Builtin {
    pub fn new(function: BuiltinFunction) -> Self {
        Builtin { function }
    }

    pub fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Vec<Object>,
}

impl Array {
    pub fn new(elements: Vec<Object>) -> Self {
        Array { elements }
    }

    pub fn inspect(&self) -> String {
        let elements: Vec<String> = self.elements.iter().map(|e| e.inspect()).collect();
        format!("[{}]", elements.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
}

#[derive(Debug, Clone)]
pub struct HashObj {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl HashObj {
    pub fn new() -> Self {
        HashObj {
            pairs: HashMap::new(),
        }
    }

    pub fn inspect(&self) -> String {
        let pairs: Vec<String> = self
            .pairs
            .values()
            .map(|pair| format!("{}: {}", pair.key.inspect(), pair.value.inspect()))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            Some(obj.clone())
        } else if let Some(ref outer) = self.outer {
            outer.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }
}