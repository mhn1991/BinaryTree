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
                //break;
            }
            Some(val) => {
                // stop case
                if val < value && self.right.is_none() {
                    self.right = Some(arena.alloc(Tree {
                        left: None,
                        value: Some(value),
                        right: None,
                    }));
                    //break;
                }
                if val < value && self.right.is_some() {
                    match self.right {
                        Some(ref mut tmp) => {
                            tmp.insert(arena, value);
                        }
                        None => todo!(),
                    }
                }
            }
        }
    }
}

fn main() {
    let arena = Arena::new();
    let mut b = Tree::init();
    //Tree::insert(&mut b,&arena,1);
    //Tree::insert(&mut b,&arena,2);
    //Tree::insert(&mut b,&arena,3);
    b.insert(&arena, 1);
    b.insert(&arena, 2);
    b.insert(&arena, 3);
    println!("{:?}", b);
}
