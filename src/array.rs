use std::{fmt, mem::take, slice, sync::Arc};

use crate::value::Value;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Array(Arc<Vec<Value>>);

impl Array {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }
    pub fn push(&mut self, value: Value) {
        Arc::make_mut(&mut self.0).push(value)
    }
    pub fn pop(&mut self) -> Option<Value> {
        Arc::make_mut(&mut self.0).pop()
    }
    pub fn iter(&self) -> slice::Iter<Value> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> slice::IterMut<Value> {
        Arc::make_mut(&mut self.0).iter_mut()
    }
    pub fn take_inner(&mut self) -> Vec<Value> {
        take(Arc::make_mut(&mut self.0))
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = Box<dyn Iterator<Item = Value>>;
    fn into_iter(self) -> Self::IntoIter {
        match Arc::try_unwrap(self.0) {
            Ok(vec) => Box::new(vec.into_iter()),
            Err(arc) => Box::new((*arc).clone().into_iter()),
        }
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = slice::Iter<'a, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = slice::IterMut<'a, Value>;
    fn into_iter(self) -> Self::IntoIter {
        Arc::make_mut(&mut self.0).iter_mut()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Extend<Value> for Array {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        Arc::make_mut(&mut self.0).extend(iter)
    }
}
