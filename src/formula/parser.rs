use pest::error::Error;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

use crate::formula::{Expr, Op};

#[derive(pest_derive::Parser)]
#[grammar = "formula/formula.pest"]
pub struct FormulaParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(unary_minus))
        .op(Op::postfix(EOI))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::EOI => lhs,
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn parse(source: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    FormulaParser::parse(Rule::equation, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _compose_pair_str(output: &mut String, lv: i32, pairs: Pairs<Rule>) {
        for pair in pairs {
            let value = pair.as_str().trim();
            if value.is_empty() {
                continue;
            }

            let rule = pair.as_rule();
            let mut i = 0;
            while i < lv {
                output.push_str("--");
                i += 1;
            }

            output.push_str(&format!("[{:?}]", rule));
            if rule == Rule::integer {
                output.push_str(&format!(" {}", value));
            }

            let pairs_new = pair.into_inner();
            _compose_pair_str(output, lv + 1, pairs_new);
        }
    }

    fn _parse_exp(lines: &str) -> String {
        let mut output = String::new();
        match parse(lines) {
            Ok(x) => {
                _compose_pair_str(&mut output, 0, x);
            }
            Err(e) => {
                println!("parse error: {:?}", e);
                assert!(false);
            }
        }
        output
    }

    #[test]
    fn basics() {
        assert_eq!(
            _parse_exp("1+1"),
            "[expr]\
	     --[integer] 1\
	     --[add]\
	     --[integer] 1"
        );
    }
}
