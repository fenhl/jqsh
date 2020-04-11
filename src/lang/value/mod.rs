use {
    std::{
        fmt,
        hash,
        iter::FromIterator,
        string
    },
    num::BigRational,
    unicode::UString
};

pub mod array;
pub mod object;

pub use self::{
    array::Array,
    object::Object
};

#[derive(Clone, Debug)]
pub enum Value {
    Exception(UString, Object<HashableValue, Value>),
    Null,
    Boolean(bool),
    Number(BigRational),
    String(UString),
    Array(Array<Value>),
    Object(Object<HashableValue, Value>),
    Function //TODO Function(Function)
}

#[derive(Clone, Debug)]
pub enum HashableValue {
    Exception(UString, Object<HashableValue, Value>),
    Null,
    Boolean(bool),
    Number(BigRational),
    String(UString),
    Array(Array<HashableValue>),
    Object(Object<HashableValue, HashableValue>)
}

impl fmt::Display for Value {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Value::*;

        match *self {
            Exception(ref name, ref meta) => {
                write!(w, "raise {:?}", string::String::from(name))?;
                if meta.len() > 0 {
                    write!(w, " {{")?;
                    for (i, (k, v)) in meta.iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        k.fmt(w)?;
                        write!(w, ": ")?;
                        v.fmt(w)?;
                    }
                    write!(w, "}}")?;
                }
            }
            Null => {
                write!(w, "null")?;
            }
            Boolean(b) => {
                write!(w, "{}", if b { "true" } else { "false" })?;
            }
            Number(ref n) => {
                write!(w, "{}", n)?;
            }
            String(ref s) => {
                write!(w, "{:?}", string::String::from(s))?;
            }
            Array(ref a) => {
                write!(w, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    item.fmt(w)?;
                }
                write!(w, "]")?;
            }
            Object(ref o) => {
                write!(w, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    k.fmt(w)?;
                    write!(w, ": ")?;
                    v.fmt(w)?;
                }
                write!(w, "}}")?;
            }
            Function => {
                write!(w, "def (...)")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for HashableValue {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        Value::from(self).fmt(w)
    }
}

impl From<HashableValue> for Value {
    fn from(v: HashableValue) -> Value {
        match v {
            HashableValue::Exception(name, meta) => {
                Value::Exception(name, Object::from_iter(meta))
            }
            HashableValue::Null => {
                Value::Null
            }
            HashableValue::Boolean(b) => {
                Value::Boolean(b)
            }
            HashableValue::Number(n) => {
                Value::Number(n)
            }
            HashableValue::String(s) => {
                Value::String(s)
            }
            HashableValue::Array(a) => {
                Value::Array(Array::from_iter(a))
            }
            HashableValue::Object(o) => {
                Value::Object(Object::from_iter(o))
            }
        }
    }
}

impl<'a> From<&'a HashableValue> for Value {
    fn from(v: &HashableValue) -> Value {
        match *v {
            HashableValue::Exception(ref name, ref meta) => {
                Value::Exception(name.clone(), Object::from_iter(meta.clone()))
            }
            HashableValue::Null => {
                Value::Null
            }
            HashableValue::Boolean(b) => {
                Value::Boolean(b)
            }
            HashableValue::Number(ref n) => {
                Value::Number(n.clone())
            }
            HashableValue::String(ref s) => {
                Value::String(s.clone())
            }
            HashableValue::Array(ref a) => {
                Value::Array(Array::from_iter(a.clone()))
            }
            HashableValue::Object(ref o) => {
                Value::Object(Object::from_iter(o.clone()))
            }
        }
    }
}

impl hash::Hash for HashableValue {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        use self::HashableValue::*;

        match *self {
            Exception(ref s, _) => {
                0.hash(state);
                s.hash(state);
            }
            Null => {
                1.hash(state);
            }
            Boolean(ref b) => {
                2.hash(state);
                b.hash(state);
            }
            Number(ref n) => {
                3.hash(state);
                n.hash(state);
            }
            String(ref s) => {
                4.hash(state);
                s.hash(state);
            }
            Array(ref a) => {
                5.hash(state);
                a.hash(state);
            }
            Object(ref o) => {
                6.hash(state);
                o.hash(state);
            }
        }
    }
}

impl PartialEq for HashableValue {
    fn eq(&self, other: &Self) -> bool {
        use self::HashableValue::*;

        match (self, other) {
            (&Exception(ref sl, _), &Exception(ref sr, _)) => sl == sr,
            (&Null, &Null) => true,
            (&Boolean(bl), &Boolean(br)) => bl == br,
            (&Number(ref nl), &Number(ref nr)) => nl == nr,
            (&String(ref sl), &String(ref sr)) => sl == sr,
            (&Array(ref al), &Array(ref ar)) => al == ar,
            (&Object(ref ol), &Object(ref or)) => ol == or,
            (_, _) => false
        }
    }
}

impl Eq for HashableValue {}

#[test]
fn test_values() {
    use std::collections;

    let mut array_map: collections::HashMap<Array<HashableValue>, &str> = collections::HashMap::new();
    array_map.insert(Array::from(vec![HashableValue::Boolean(false), HashableValue::Null]), "test 1");
    array_map.insert(Array::new(), "test 2");
}
