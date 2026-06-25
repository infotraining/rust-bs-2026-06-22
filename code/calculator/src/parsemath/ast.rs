#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Negate(Box<Expression>),
    Grouping(Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Add(a, b) => a.evaluate() + b.evaluate(),
            Expression::Subtract(a, b) => a.evaluate() - b.evaluate(),
            Expression::Multiply(a, b) => a.evaluate() * b.evaluate(),
            Expression::Divide(a, b) => a.evaluate() / b.evaluate(),
            Expression::Negate(expr) => -expr.evaluate(),
            Expression::Grouping(expr) => expr.evaluate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::parsemath::ast::Expression;

    #[rstest]
    #[case::ast_add_1_2(
        Expression::Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Number(2.0))
        ), 3.0)]
    #[case::ast_mul_minus_1_2(
        Expression::Multiply(
            Box::new(Expression::Negate(Box::new(Expression::Number(1.0)))),
            Box::new(Expression::Number(2.0))
        ), -2.0)]
    #[case::ast_group_add_1_2_mul_group_sub_3_4(
        Expression::Multiply(
            Box::new(Expression::Grouping(
                Box::new(Expression::Add(
                    Box::new(Expression::Number(1.0)),
                    Box::new(Expression::Number(2.0))
                ))
            )),
            Box::new(Expression::Grouping(
                Box::new(Expression::Subtract(
                    Box::new(Expression::Number(3.0)),
                    Box::new(Expression::Number(4.0))
                ))
            ))
        ), -3.0)]
    fn eval_simple_expression(#[case] ast: Expression, #[case] expected: f64)
    {
        let result = ast.evaluate();

        assert_eq!(result, expected);
    }
}