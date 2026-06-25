use crate::parsemath::parser::Parser;
use mockall::automock;

#[automock]
pub trait Console {
    fn readline(&self) -> String;
    fn println(&self, text: &str);
    fn print(&self, text: &str);
}

pub struct Calculator<'a> {
    console: &'a dyn Console,
}

impl<'a> Calculator<'a> {
    pub fn new(console: &'a dyn Console) -> Calculator<'a> {
        Calculator { console }
    }

    pub fn run(&mut self) {
        self.console.println("### Calculator ver. 1.0 ###");

        loop {
            self.console.print(">>> ");
            let input = self.console.readline().to_uppercase();

            if input == "EXIT" {
                break;
            }

            match Parser::new(&input)  {
                Ok(mut parser) => {
                    match parser.parse() {
                        Ok(ast) => {
                            let result = ast.evaluate();
                            self.console.println(&format!("{}", result));
                        }
                        Err(error) => {
                            self.console.println(&format!("{}", error));
                        }
                    }
                }
                Err(error) => {
                    self.console.println(&format!("{}", error));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::*;

    use crate::calcmath::calc::{Calculator, MockConsole};

    #[test]
    fn calculator_run_loop_evaluates_expressions() {
        let mut mock_console = MockConsole::new();

        let mut seq = Sequence::new();

        mock_console
            .expect_println()
            .with(eq("### Calculator ver. 1.0 ###"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());

        mock_console.expect_print()
            .with(eq(">>> "))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());

        mock_console
            .expect_readline()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| "1 + 2".to_string());

        mock_console
            .expect_println()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq("3"))
            .returning(|_| ());

        mock_console.expect_print()
            .with(eq(">>> "))
            .times(1)
            .in_sequence(&mut seq)
            .return_once(|_| ());

        mock_console
            .expect_readline()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| "(1 + 2) * (10 / 5)".to_string());

        mock_console
            .expect_println()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq("6"))
            .returning(|_| ());

        mock_console.expect_print()
            .with(eq(">>> "))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());

        mock_console.expect_readline()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| "Exit".to_string());


        let mut calculator = Calculator::new(&mock_console);

        calculator.run();
    }

    #[test]
    fn calculator_run_loop_handles_parser_errors() {
        let mut mock_console = MockConsole::new();

        let mut seq = Sequence::new();

        mock_console
            .expect_println()
            .with(eq("### Calculator ver. 1.0 ###"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());

        mock_console.expect_print()
            .with(eq(">>> "))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());

        mock_console
            .expect_readline()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| "2#".to_string());

        mock_console
            .expect_println()
            .times(1)
            .in_sequence(&mut seq)
            .with(eq("Syntax error: Unexpected token \'#\'"))
            .returning(|_| ());

        mock_console.expect_print()
            .with(eq(">>> "))
            .times(1)
            .in_sequence(&mut seq)
            .return_once(|_| ());

        mock_console.expect_readline()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| "Exit".to_string());


        let mut calculator = Calculator::new(&mock_console);

        calculator.run();
    }
}
