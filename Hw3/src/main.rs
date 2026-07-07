#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Expression {
    Op { op : Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    match e {
        Expression::Value(v) => v,
        Expression::Op { op, left, right } => {
            let l = eval(*left);
            let r = eval(*right);
            match op {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => {if r == 0 {println!("Error"); 0} else {l/r}},
            }
        }
    }
}

fn main() {
    let expr1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(2)),
        }),
        right: Box::new(Expression::Value(9)),
    };

    println!("Result = {}", eval(expr1));

    let expr2 = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Op { 
                    op: Operation::Mul, 
                    left: Box::new(Expression::Value(5)), 
                    right: Box::new(Expression::Value(10)) }),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(2)),
        }),
        right: Box::new(Expression::Value(7)),
    };

    println!("Result2 = {}", eval(expr2));

}