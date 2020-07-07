use math_ast::AST;

#[test]
fn test() {
    println!("{}", AST::Number(1) + AST::Number(2));
    println!("{}", AST::Number(1) - AST::Number(2))
}
