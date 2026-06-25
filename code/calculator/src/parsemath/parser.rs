use crate::parsemath::ast::Expression;
use crate::parsemath::token::Token;
use crate::parsemath::tokenizer::{Tokenizer, TokenizingError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParserError {
    #[error("Syntax error: {0}")]
    UnexpectedToken(#[from] TokenizingError),
    #[error("Syntax error: {0}")]
    SyntaxError(String)
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
    bracket_count: usize,
}

// A simple recursive descent parser for mathematical expressions.
// Write grammar in EBNF:
// expression = term;
// term       = factor ( ( "-" | "+" ) factor )* ;
// factor     = unary ( "/" | "*" ) unary )* ;
// unary      = ( "-" )* unary | primary ;
// primary    = NUMBER | "(" expression ")" ;

impl Parser {
    pub fn new(expression: &str) -> Result<Self, ParserError> {
        let tokenizer = Tokenizer::new(expression);
        let tokens = tokenizer.collect::<Result<Vec<Token>, TokenizingError>>()?;

        Ok(Parser {
            tokens,
            current_token_index: 0,
            bracket_count: 0,
        })
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        let expression = self.expression();
        expression
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        let expression = self.term();

        expression
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.factor()?;

        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.consume();
                    let right = self.factor()?;
                    expression = Expression::Add(Box::new(expression), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.consume();
                    let right = self.factor()?;
                    expression = Expression::Subtract(Box::new(expression), Box::new(right));
                }
                Some(Token::RightParen) if self.bracket_count == 0 => {
                    return Err(ParserError::SyntaxError(r#"Too many ')'."#.to_string()));
                },
                Some(Token::LeftParen) if self.bracket_count == 0 => {
                    return Err(ParserError::SyntaxError(r#"Unexpected '('."#.to_string()));
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.unary()?;

        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.consume();
                    let right = self.unary()?;
                    expression = Expression::Multiply(Box::new(expression), Box::new(right));
                }
                Some(Token::Slash) => {
                    self.consume();
                    let right = self.unary()?;
                    expression = Expression::Divide(Box::new(expression), Box::new(right));
                },
                Some(Token::RightParen) if self.bracket_count == 0 => {
                    return Err(ParserError::SyntaxError(r#"Too many ')'."#.to_string()));
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if let Some(Token::Minus) = self.peek() {
            self.consume();

            let right = self.unary()?;
            return Ok(Expression::Negate(Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let expression = match self.next() {
            Some(Token::Number(n)) => Expression::Number(n),
            Some(Token::LeftParen) => {
                self.bracket_count += 1;
                let expression = self.expression()?;
                self.consume_right_paren()?;
                Expression::Grouping(Box::new(expression))
            }
            _ => return Err(ParserError::SyntaxError("Expected number or '('.".to_string())),
        };

        Ok(expression)
    }

    fn consume(&mut self) {
        if !self.is_at_end() {
            self.current_token_index += 1;
        }
    }

    fn consume_right_paren(&mut self) -> Result<(), ParserError> {
        if let Some(Token::RightParen) = self.next() {
            self.bracket_count -= 1;
            return Ok(());
        }
        else
        {
            Err(ParserError::SyntaxError("Expect ')' after expression.".to_string()))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_token_index >= self.tokens.len()
    }

    fn peek(&self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }
        Some(self.tokens[self.current_token_index].clone())
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.current_token_index += 1;
        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::parsemath::ast::Expression;
    use crate::parsemath::parser::{Parser, ParserError};
    use rstest::*;
    use crate::parsemath::tokenizer::TokenizingError;

    #[rstest]
    fn parse_to_ast() {
        let expression = "1";
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, Expression::Number(1.0));
    }

    #[rstest]
    #[case::expr_1_times_2(
        "1 * 2",
        Expression::Multiply(Box::new(Expression::Number(1.0)), Box::new(Expression::Number(2.0)))
    )]
    #[case::expr_1_times_2_times_3(
        "1 * 2 * 3",
        Expression::Multiply(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )),
            Box::new(Expression::Number(3.0))
        )
    )]
    fn parse_multiplication(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    fn parse_division() {
        let expression = "1 / 2";
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            Expression::Divide(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[rstest]
    #[case::expr_1_plus_2(
        "1 + 2",
        Expression::Add(Box::new(Expression::Number(1.0)), Box::new(Expression::Number(2.0)))
    )]
    fn parse_addition(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_1_minus_2(
        "1 - 2",
        Expression::Subtract(Box::new(Expression::Number(1.0)), Box::new(Expression::Number(2.0)))
    )]
    fn parse_subtraction(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_1_plus_2_minus_3(
        "1 + 2 - 3",
        Expression::Subtract(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )),
            Box::new(Expression::Number(3.0))
        )
    )]
    #[case::expr_2_times_4_plus_6_div_2(
        "2 * 4 + 6 / 2",
        Expression::Add(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(4.0))
            )),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(6.0)),
                Box::new(Expression::Number(2.0))
            ))
        )
    )]
    fn parse_complex_expression(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_negate_1("-1", Expression::Negate(Box::new(Expression::Number(1.0))))]
    #[case::expr_negate_1_plus_2(
        "-1 + 2",
        Expression::Add(
            Box::new(Expression::Negate(Box::new(Expression::Number(1.0)))),
            Box::new(Expression::Number(2.0))
        )
    )]
    fn parse_negate_expression(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::expr_lb_1_rb("(1)", Expression::Grouping(Box::new(Expression::Number(1.0))))]
    #[case::expr_lb_1_plus_2_rb_times_lb_3_minus_4_rb(
        "(1 + 2) * (3 - 4)",
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
            )),
        )
    )]
    fn parse_expression_with_brackets(#[case] expression: &str, #[case] expected_ast: Expression) {
        let mut parser = Parser::new(expression).unwrap();

        let ast = parser.parse().unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[rstest]
    #[case::lb_1("(1")]
    #[case::lb_lb_lb_1_rb_2_rb("(((1)2)")]
    fn parse_unclosed_bracket(#[case] expression: &str) {
        let mut parser = Parser::new(expression).unwrap();
        let parser_error = parser.parse().unwrap_err();

        assert_eq!(parser_error, ParserError::SyntaxError(r#"Expect ')' after expression."#.to_string()));
    }

    #[rstest]
    #[case::lb_1_plus_2_rb_rb("(1))")]
    #[case::lb_1_plus_2_rb_rb("(1))+2")]
    fn parse_too_many_closing_brackets(#[case] expression: &str) {
        let mut parser = Parser::new(expression).unwrap();
        let parser_error = parser.parse().unwrap_err();

        assert_eq!(parser_error, ParserError::SyntaxError(r#"Too many ')'."#.to_string()));
    }

    #[rstest]
    #[case::expr_plus_plus("++")]
    #[case::expr_1_minus("1-")]
    #[case::expr_rb_1(")1")]
    fn parse_invalid_expression(#[case] expression: &str) {
        let mut parser = Parser::new(expression).unwrap();
        let parser_error = parser.parse().unwrap_err();
        assert_eq!(parser_error, ParserError::SyntaxError(r#"Expected number or '('."#.to_string()));
    }

    #[rstest]
    fn parsing_error_unexpected_tokens() {
        let expr = "2#";
        let parser_error = Parser::new(expr).unwrap_err();
        assert_eq!(parser_error, ParserError::UnexpectedToken(TokenizingError::InvalidCharacter('#')));
        assert_eq!(format!("{}", parser_error), "Syntax error: Unexpected token \'#\'");
    }

    #[rstest]
    fn bug() {
        let expr = "(1)(";
        let mut parser = Parser::new(expr).unwrap();
        let parser_error = parser.parse().unwrap_err();
        assert_eq!(parser_error, ParserError::SyntaxError("Unexpected \'(\'.".to_string()));

    }
}
