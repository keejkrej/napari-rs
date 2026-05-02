use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ContextValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    StringList(Vec<String>),
    None,
}

impl From<bool> for ContextValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for ContextValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<usize> for ContextValue {
    fn from(value: usize) -> Self {
        Self::Int(value as i64)
    }
}

impl From<f64> for ContextValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<&str> for ContextValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ContextValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

pub enum ContextEntry {
    Value(ContextValue),
    Lazy(Box<dyn Fn() -> ContextValue>),
}

impl From<ContextValue> for ContextEntry {
    fn from(value: ContextValue) -> Self {
        Self::Value(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextKeyError {
    key: String,
}

impl ContextKeyError {
    pub fn key(&self) -> &str {
        &self.key
    }
}

impl fmt::Display for ContextKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Key {:?} not found", self.key)
    }
}

impl std::error::Error for ContextKeyError {}

#[derive(Default)]
pub struct ContextMapping {
    initial: BTreeMap<String, ContextEntry>,
    evaluated: BTreeMap<String, ContextValue>,
}

impl ContextMapping {
    pub fn new(initial: impl IntoIterator<Item = (String, ContextEntry)>) -> Self {
        Self {
            initial: initial.into_iter().collect(),
            evaluated: BTreeMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.initial.len()
    }

    pub fn is_empty(&self) -> bool {
        self.initial.is_empty()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.initial.contains_key(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.initial.keys().map(String::as_str)
    }

    pub fn get(&mut self, key: &str) -> Result<&ContextValue, ContextKeyError> {
        if !self.evaluated.contains_key(key) {
            let value = match self.initial.get(key) {
                Some(ContextEntry::Value(value)) => value.clone(),
                Some(ContextEntry::Lazy(getter)) => getter(),
                None => {
                    return Err(ContextKeyError {
                        key: key.to_string(),
                    });
                }
            };
            self.evaluated.insert(key.to_string(), value);
        }
        Ok(self
            .evaluated
            .get(key)
            .expect("context value was inserted or already cached"))
    }
}
