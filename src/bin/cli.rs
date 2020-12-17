// use cel_rust::{IdentParser, Rule};
use common_expression_language::cel;
use common_expression_language::ast::Expression;
use std::mem::size_of;

fn main() {
    println!("{:?}", cel::ExpressionParser::new().parse("b(c + 3 / 2,) == 2").unwrap_or_else(|e| panic!("{}", e)));
    println!("{:?}", cel::ExpressionParser::new().parse(r"has(account.properties.id) && (type(account.properties.id) == string || type(account.properties.id) == list(string))").unwrap_or_else(|e| panic!("{}", e)));
    println!("{:?}", cel::ExpressionParser::new().parse("-1E3").unwrap_or_else(|e| panic!("{}", e)));
    println!("{:?}", cel::ExpressionParser::new().parse("[1,2,3, abc, ]").unwrap_or_else(|e| panic!("{}", e)));
    println!("{:?}", cel::ExpressionParser::new().parse("Account{user_id: 123}").unwrap_or_else(|e| panic!("{}", e)));
    println!("{:?}", cel::ExpressionParser::new().parse("{1 + a: 3}").unwrap_or_else(|e| panic!("{}", e)));
    println!("{:?}", cel::ExpressionParser::new().parse("Account{user_id: 123}.user_id == 123").unwrap_or_else(|e| panic!("{}", e)));
    println!("{}", size_of::<Expression>())
    // println!("{:?}", cel::ExpressionParser::new().parse("1 && (2 + 10) ? 10 : 20").unwrap_or_else(|e| panic!("{}", e)));
    // println!("{:?}", cel::ExpressionParser::new().parse("!1 ? 10 : 20").unwrap_or_else(|e| panic!("{}", e)));
    // let pairs = IdentParser::parse(Rule::multiplication, "1 * 1")
    // println!("{:?}", pairs);
}