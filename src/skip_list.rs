pub mod cli {
    pub fn main() {
        println!("not implemented");
    }
}

use std::cmp::{PartialEq, PartialOrd};

#[derive(PartialOrd, PartialEq)]
#[repr(isize)]
enum NodeKey<T> {
    Head = isize::MIN,
    Sentinal = isize::MAX,
    Value(T) = 0,
}

struct Node<T, U>
where
    T: PartialOrd,
{
    key: NodeKey<T>,
    value: Option<U>,
    next: Vec<Box<Self>>,
}

impl<T, U> Node<T, U>
where
    T: PartialOrd,
{
    pub fn new(key: NodeKey<T>, value: Option<U>, levels: usize) -> Self {
        Node::<T, U> {
            key,
            value,
            next: Vec::new(),
        }
    }
    pub fn get_next(&self, level: usize) -> Option<&Box<Self>> {
        self.next.get(level)
    }
    pub fn get_key(&self) -> &NodeKey<T> {
        &self.key
    }
    pub fn get_value(&self) -> &Option<U> {
        &self.value
    }
    pub fn get_levels(&self) -> usize {
        self.next.len()
    }
    pub fn set_next(&mut self, level: usize, next: Self) -> () {
        self.next.insert(level, Box::new(next));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init() {
        assert!(NodeKey::Head::<isize> < NodeKey::Sentinal);
        assert!(NodeKey::Head < NodeKey::Value(5));
        assert!(NodeKey::Head < NodeKey::Value(-5555));
        assert!(NodeKey::Value(5) < NodeKey::Sentinal);
        assert!(NodeKey::Value(5555) < NodeKey::Sentinal);
    }
}
