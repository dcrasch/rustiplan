use crate::formula::ast::{Expr, Op};

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Integer(n) => *n,
        Expr::UnaryMinus(expr) => -eval(expr),
        Expr::BinaryExpr { lhs, op, rhs } => {
            let lhs = eval(&lhs);
            let rhs = eval(&rhs);

            match op {
                Op::Add => lhs + rhs,
                Op::Subtract => lhs - rhs,
                Op::Multiply => lhs * rhs,
                Op::Divide => lhs / rhs,
                Op::Modulo => lhs % rhs,
            }
        }
    }
}
