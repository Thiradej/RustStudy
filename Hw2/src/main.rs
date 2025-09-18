enum Node {
    Nil,
    Cons(i32, Box<Node>),
}

struct Stack {
    top: Node,
}

impl Stack {
    fn new() -> Self {
        Stack { top: Node::Nil }
    }

    fn push(&mut self, value: i32) {
        println!("Pushing {} onto the stack.", value);
        self.top = Node::Cons(value, Box::new(std::mem::replace(&mut self.top, Node::Nil)));
    }

    fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.top, Node::Nil) {
            Node::Nil => None,
            Node::Cons(value, next) => {
                println!("Popped {} from the stack.", value);
                self.top = *next;
                Some(value)
            }
        }
    }

    fn backnumber(&self) -> Option<i32> {
        match &self.top {
            Node::Nil => None,
            Node::Cons(v, _) => Some(*v),
        }
    }

    fn check_empty(&self) -> bool {
        matches!(self.top, Node::Nil)
    }

    fn print_stack(&self) {
        print!("Stack contents: ");
        Self::print_recursive(&self.top);
        println!();
    }

    fn print_recursive(node: &Node) {
        match node {
            Node::Nil => print!("Nil"),
            Node::Cons(v, next) => {
                print!("{} -> ", v);
                Self::print_recursive(next);
            }
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);
    stack.print_stack();

    println!("Top of the stack: {}", stack.backnumber().unwrap());

    stack.pop();
    stack.print_stack();

    stack.pop();
    stack.print_stack();

    stack.pop();
    stack.print_stack();

    println!("Is the stack empty? {}", stack.check_empty());
}