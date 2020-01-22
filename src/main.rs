extern crate typed_arena;
use typed_arena::Arena;

#[derive(Debug)]
struct Tree<'a, T> {
    left: Option<&'a mut Tree<'a, T>>,
    value: Option<T>,
    right: Option<&'a mut Tree<'a, T>>,
}

impl<'a> Tree<'a, i32> {
    fn init() -> Tree<'a, i32> {
        Tree {
            left: None,
            value: None,
            right: None,
        }
    }

    fn insert(&mut self, arena: &'a Arena<Tree<'a, i32>>, value: i32) {
        match self.value {
            None => {
                self.value = Some(value);
            }
            Some(val) => {
                // right insertion stop case
                if val < value && self.right.is_none() {
                    self.right = Some(arena.alloc(Tree {
                        left: None,
                        value: Some(value),
                        right: None,
                    }));
                }
                // moving right
                if val < value && self.right.is_some() {
                    match self.right {
                        Some(ref mut tmp) => {
                            tmp.insert(arena, value);
                        }
                        None => println!("how come !?"),
                    }
                }
                // left insertion stop case
                if val > value && self.left.is_none() {
                    self.left = Some(arena.alloc(Tree {
                        left: None,
                        value: Some(value),
                        right: None,
                    }));
                }
                // moving left
                if val > value && self.left.is_some() {
                    match self.left {
                        Some(ref mut tmp) => {
                            tmp.insert(arena, value);
                        }
                        None => println!("how come !?"),
                    }
                }
            }
        }
    }
}

fn main() {
    let arena = Arena::new();
    let mut b = Tree::init();
    b.insert(&arena, 3);
    b.insert(&arena, 2);
    b.insert(&arena, 1);
    println!("{:?}", b);
}
