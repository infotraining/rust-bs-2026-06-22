use super::ast::Expression;

pub trait ExpressionVisitor<T, Error> {
    fn visit_number(&mut self, value: f64) -> Result<T, Error>;
    fn visit_add(&mut self, left: &Expression, right: &Expression) -> Result<T, Error>;
    fn visit_subtract(&mut self, left: &Expression, right: &Expression) -> Result<T, Error>;
    fn visit_multiply(&mut self, left: &Expression, right: &Expression) -> Result<T, Error>;
    fn visit_divide(&mut self, left: &Expression, right: &Expression) -> Result<T, Error>;
    fn visit_negate(&mut self, expr: &Expression) -> Result<T, Error>;
    fn visit_grouping(&mut self, expr: &Expression) -> Result<T, Error>;

    fn visit_expression(&mut self, expr: &Expression) -> Result<T, Error> {
        match expr {
            Expression::Number(n) => self.visit_number(*n),
            Expression::Add(a, b) => self.visit_add(a, b),
            Expression::Subtract(a, b) => self.visit_subtract(a, b),
            Expression::Multiply(a, b) => self.visit_multiply(a, b),
            Expression::Divide(a, b) => self.visit_divide(a, b),
            Expression::Negate(e) => self.visit_negate(e),
            Expression::Grouping(e) => self.visit_grouping(e) 
        }
    }
}


#[derive(Debug)]
pub struct EvaluatorError {
    message: String,
}


pub struct Evaluator;

impl ExpressionVisitor<f64, EvaluatorError> for Evaluator {
    fn visit_number(&mut self, value: f64) -> Result<f64, EvaluatorError> {
        Ok(value)
    }

    fn visit_add(&mut self, left: &Expression, right: &Expression) -> Result<f64, EvaluatorError> {
        Ok(self.visit_expression(left)? + self.visit_expression(right)?)
    }

    fn visit_subtract(&mut self, left: &Expression, right: &Expression) -> Result<f64, EvaluatorError> {
        Ok(self.visit_expression(left)? - self.visit_expression(right)?)
    }

    fn visit_multiply(&mut self, left: &Expression, right: &Expression) -> Result<f64, EvaluatorError> {
        Ok(self.visit_expression(left)? * self.visit_expression(right)?)
    }

    fn visit_divide(&mut self, left: &Expression, right: &Expression) -> Result<f64, EvaluatorError> {
        let right_value = self.visit_expression(right)?;
        if right_value == 0.0 {
            return Err(EvaluatorError { message: "Division by zero".to_string() });
        }

        Ok(self.visit_expression(left)? / self.visit_expression(right)?)
    }

    fn visit_negate(&mut self, expr: &Expression) -> Result<f64, EvaluatorError> {
        Ok(-self.visit_expression(expr)?)
    }

    fn visit_grouping(&mut self, expr: &Expression) -> Result<f64, EvaluatorError> {
        self.visit_expression(expr)
    }
}

pub struct PrettyPrinterVisitor{
    output: String,
}

impl ExpressionVisitor<String, ()> for PrettyPrinterVisitor {
    fn visit_number(&mut self, value: f64) -> Result<String, ()> {
        Ok(value.to_string())
    }

    fn visit_add(&mut self, left: &Expression, right: &Expression) -> Result<String, ()> {
        Ok(format!("{} + {}", self.visit_expression(left)?, self.visit_expression(right)?))
    }

    fn visit_subtract(&mut self, left: &Expression, right: &Expression) -> Result<String, ()> {
        Ok(format!("{} - {}", self.visit_expression(left)?, self.visit_expression(right)?))
    }

    fn visit_multiply(&mut self, left: &Expression, right: &Expression) -> Result<String, ()> {
        Ok(format!("{} * {}", self.visit_expression(left)?, self.visit_expression(right)?))
    }

    fn visit_divide(&mut self, left: &Expression, right: &Expression) -> Result<String, ()> {
        Ok(format!("{} / {}", self.visit_expression(left)?, self.visit_expression(right)?))
    }

    fn visit_negate(&mut self, expr: &Expression) -> Result<String, ()> {
        Ok(format!("-{}", self.visit_expression(expr)?))
    }

    fn visit_grouping(&mut self, expr: &Expression) -> Result<String, ()> {
        Ok(format!("({})", self.visit_expression(expr)?))
    }
}



#[cfg(test)]
mod visitor_tests {
    use rstest::{fixture, rstest};
    use super::*;
    use crate::parsemath::ast::Expression;

    #[fixture]
    fn expression() -> crate::parsemath::ast::Expression {
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
        )
    }

    #[rstest]
    fn evaluate_expression_with_visitor(expression: Expression) {
        let mut evaluator = Evaluator{};
        let result = evaluator.visit_expression(&expression);
        assert_eq!(result.unwrap(), -3.0);
    }

    #[rstest]
    fn evaluate_expression_with_visitor_div_by_zero() {
        let expression = Expression::Divide(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Number(0.0))
        );

        let mut evaluator = Evaluator{};
        let result = evaluator.visit_expression(&expression);
        assert!(result.is_err());
    }

    #[rstest]
    fn pretty_print_expression_with_visitor(expression: Expression) {
        let mut printer = PrettyPrinterVisitor{ output: String::new() };
        let result = printer.visit_expression(&expression);
        assert_eq!(result.unwrap(), "(1 + 2) * (3 - 4)");
    }
}
