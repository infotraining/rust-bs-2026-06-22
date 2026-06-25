
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Plus,
    Minus,
    Slash,
    Star,
    Caret,
    LeftParen,
    RightParen,
    Number(f64)
}