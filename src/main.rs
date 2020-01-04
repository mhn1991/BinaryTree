extern crate typed_arena;
use typed_arena::Arena;

#[derive(Debug)]
struct Tree<'a, T> {
    left: Option<&'a Tree<'a, T>>,
    value: Option<T>,
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

    fn insert(&mut self, arena: &'a Arena<Tree<'a,i32>> ,value: i32) -> bool {
        match self.value {
            None => {
                self.value = Some(value);
                return true;
            }
            Some(val) => {
                if val < value {
                    let mut tmp = arena.alloc(Tree{left:None,value:Some(value),right:None});
                    self.right = Some(tmp);
                }
                return false;
            }
        }
    }
}

fn main() {
    let arena = Arena::new();
    let mut b = Tree::init();
    b.insert(&arena,1);
    b.insert(&arena,2);
    println!("{:?}", b);
}
