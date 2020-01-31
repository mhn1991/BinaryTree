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

    fn del(&mut self, value: i32) {
        if self.left.as_ref().unwrap().value.unwrap() >= value {
	    //stop case for not founding
            if self.left.is_none() {
		println!("didn't find the given value on tree");
		return;
	    }
	    //no child
	    if self.left.as_ref().unwrap().value.unwrap() == value && self.left.as_ref().unwrap().left.is_none() && self.left.as_ref().unwrap().right.is_none(){
		self.left = None;
		return;
	    }
	    if self.left.as_ref().unwrap().value.unwrap() == value && self.left.as_ref().unwrap().left.is_some(){
		self.left.as_mut().unwrap().value = Some(self.left.as_ref().unwrap().left.as_ref().unwrap().value.unwrap());
		self.left.as_mut().unwrap().left = None;
		return;
	    }
	    if self.left.as_ref().unwrap().value.unwrap() == value && self.left.as_ref().unwrap().right.is_some(){
                self.left.as_mut().unwrap().value = Some(self.left.as_ref().unwrap().right.as_ref().unwrap().value.unwrap());
                self.left.as_mut().unwrap().right = None;
                return;                                                                                                                
            } 
	    self.left.as_mut().unwrap().del(value);
        }
    }
}

fn main() {
    let arena = Arena::new();
    let mut b = Tree::init();
    b.insert(&arena, 3);
    b.insert(&arena, 2);
    b.insert(&arena, 1);
    b.del(1);
    println!("{:?}", b);
}
