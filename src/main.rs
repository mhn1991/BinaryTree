#[derive(Debug)]
struct Tree<'a, T> {
    left: Option<&'a Tree<'a, T>>,
    value: Option<&'a T>,
    right: Option<&'a Tree<'a, T>>,
}

impl<'a> Tree<'a, i32> {
    fn init() -> Tree<'a, i32> {
        Tree {
            left: None,
            value: None,
            right: None,
        }
    }

    fn insert(&'a mut self, value: &'a i32) -> &'a Tree<'a, i32> {
        match self.value {
            // it means we should insert in root
            None => {
                self.value = Some(value);
                return self;
            }
            Some(node) => {
		//we should go right
		if node.value < value {

		}
                return self;
            }
        }
    }
}

fn main() {
    let mut b = Tree::init();
    println!("{:?}", b.insert(&1));
}
