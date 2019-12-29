struct Tree<'a,T>{
    left : Option<&'a mut Node<T>>,
    node : Option<&'a mut Node<T>>,
    right : Option<&'a mut Node<T>>
}

struct Node<T>{
    data: T
}


impl Node<i32>{
    fn set(value:i32) -> Node<i32>{
	let node = Node{
	    data : value,
	};
	node
    }
}


impl<'a> Tree<'a,i32>{
    fn setRoot(node: &'a mut Node<i32>) -> Tree<'a,i32>{
	let tree = Tree{
	    left : None,
	    node: Some(node),
	    right:None,
	};
	tree
    }
}
fn main() {
    let mut b = Tree::setRoot(&mut (Node::set(1)));
}
