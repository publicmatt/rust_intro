use std::fmt;

struct Tree {
    label: String,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}
impl Tree {
    pub fn new(label: &str, left: Option<Box<Tree>>, right: Option<Box<Tree>>) -> Self {
        Tree {
            label: label.to_string(),
            left,
            right,
        }
    }
    pub fn in_order(&self) -> Vec<&Tree> {
        let mut out: Vec<&Tree> = Vec::new();
        match self.left {
            Some(ref left) => {
                out.extend(left.in_order());
            }
            None => (),
        }
        out.extend(vec![self]);
        match self.right {
            Some(ref right) => {
                out.extend(right.in_order());
            }
            None => (),
        }
        out
    }
    pub fn print(&self, depth: usize) -> String {
        let mut out: Vec<String> = Vec::new();
        match self.left {
            Some(ref left) => {
                out.extend(vec![left.print(depth + 1)]);
            }
            None => (),
        }
        out.extend(vec!["\t".repeat(depth).to_string() + &self.label]);
        match self.right {
            Some(ref right) => {
                out.extend(vec![right.print(depth + 1)]);
            }
            None => (),
        }
        out.join("\n")
    }
    pub fn add(&self, tree: Tree) {
        ()
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(0))
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
        assert_eq!(t.left.unwrap().label, "left");
    }
    #[test]
    fn test_print() {
        let mut left = Tree::new("left", None, None);
        let right = Tree::new("right", None, None);
        let t = Tree::new("root", Some(Box::new(left)), Some(Box::new(right)));
        assert_eq!(format!("{}", t), "\tleft\nroot\n\tright");
    }
}
