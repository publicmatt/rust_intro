use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tree<T>
where
    T: Display + PartialOrd,
{
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}
impl<T> Tree<T>
where
    T: Display + PartialOrd,
{
    pub fn new(value: T, left: Option<Box<Tree<T>>>, right: Option<Box<Tree<T>>>) -> Self {
        Tree { value, left, right }
    }
    pub fn in_order(&self, depth: usize) -> Vec<(&Tree<T>, usize)> {
        let mut out: Vec<(&Tree<T>, usize)> = Vec::new();
        match self.left {
            Some(ref left) => {
                out.extend(left.in_order(depth + 1));
            }
            None => (),
        }
        out.extend(vec![(self, depth)]);
        match self.right {
            Some(ref right) => {
                out.extend(right.in_order(depth + 1));
            }
            None => (),
        }
        out
    }
    pub fn print(&self) -> String {
        let ordered = self.in_order(0);
        let out: Vec<String> = ordered
            .iter()
            .map(|(node, d)| format!("{}{}", "\t".repeat(*d), &node.value))
            .collect();
        // let mut out: Vec<String> = Vec::new();
        // match self.left {
        //     Some(ref left) => {
        //         out.extend(vec![left.print(depth + 1)]);
        //     }
        //     None => (),
        // }
        // out.extend(vec!["\t".repeat(depth).to_string() + &self.label]);
        // match self.right {
        //     Some(ref right) => {
        //         out.extend(vec![right.print(depth + 1)]);
        //     }
        //     None => (),
        // }
        out.join("\n")
    }
    pub fn add(&mut self, node: Tree<T>) {
        let next: &mut Option<Box<Tree<T>>>;
        if node.value <= self.value {
            next = &mut self.left;
        } else {
            next = &mut self.right;
        }
        match next {
            Some(ref mut next) => {
                next.add(node);
            }
            None => {
                *next = Some(Box::new(node));
            }
        }
    }
}

impl<T> fmt::Display for Tree<T>
where
    T: Display + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}
pub mod cli {
    pub fn main() {
        println!("not implemented");
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init() {
        let left = Tree::new("left", None, None);
        let right = Tree::new("right", None, None);
        let t = Tree::new("root", Some(Box::new(left)), Some(Box::new(right)));
        assert_eq!(t.left.unwrap().value, "left");
    }
    #[test]
    fn test_generic() {
        let left = Tree::new(5, None, None);
        let right = Tree::new(10, None, None);
        let t = Tree::new(0, Some(Box::new(left)), Some(Box::new(right)));
        assert_eq!(t.left.unwrap().value, 5);
    }
    #[test]
    fn test_print() {
        let mut left = Tree::new("left", None, None);
        let right = Tree::new("right", None, None);
        let t = Tree::new("root", Some(Box::new(left)), Some(Box::new(right)));
        assert_eq!(format!("{}", t), "\tleft\nroot\n\tright");
    }
    #[test]
    fn test_add() {
        let left = Tree::new(-5, None, None);
        let right = Tree::new(10, None, None);
        let mut t = Tree::new(0, None, None);
        t.add(left);
        t.add(right);
        assert_eq!(format!("{}", t), "\t-5\n0\n\t10");
    }
}
