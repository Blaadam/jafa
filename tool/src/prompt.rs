use anyhow::Result;
use std::io::{self, BufRead, IsTerminal, Write};

/// Asks a yes/no question. Without a terminal to answer on, it returns `default` instead of hanging
pub fn confirm(question: &str, default: bool) -> bool {
    if !io::stdin().is_terminal() {
        return default;
    }

    confirm_from(&mut io::stdin().lock(), question, default)
}

/// Asks for a line of text, asking again until `validate` accepts it. Enter gives `default`, and
/// without a terminal to answer on it returns `default` instead of hanging
pub fn input(question: &str, default: &str, validate: impl Fn(&str) -> Result<()>) -> String {
    if !io::stdin().is_terminal() {
        return default.to_string();
    }

    input_from(&mut io::stdin().lock(), question, default, validate)
}

fn input_from(
    input: &mut impl BufRead,
    question: &str,
    default: &str,
    validate: impl Fn(&str) -> Result<()>,
) -> String {
    loop {
        print!("{question} [{default}] ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) | Err(_) => return default.to_string(),
            Ok(_) => {}
        }

        let answer = line.trim();
        if answer.is_empty() {
            return default.to_string();
        }

        match validate(answer) {
            Ok(()) => return answer.to_string(),
            Err(err) => println!("{err}"),
        }
    }
}

fn confirm_from(input: &mut impl BufRead, question: &str, default: bool) -> bool {
    let hint = if default { "Y/n" } else { "y/N" };

    loop {
        print!("{question} [{hint}] ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) | Err(_) => return default,
            Ok(_) => {}
        }

        match line.trim().to_lowercase().as_str() {
            "" => return default,
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer y or n."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{confirm_from, input_from};
    use anyhow::{Result, bail};
    use std::io::Cursor;

    fn answer(input: &str, default: bool) -> bool {
        confirm_from(&mut Cursor::new(input), "?", default)
    }

    #[test]
    fn yes_and_no() {
        assert!(answer("y\n", false));
        assert!(answer("YES\n", false));
        assert!(!answer("n\n", true));
        assert!(!answer("No\n", true));
    }

    #[test]
    fn empty_line_and_eof_use_default() {
        assert!(answer("\n", true));
        assert!(!answer("\n", false));
        assert!(answer("", true));
    }

    #[test]
    fn asks_again_on_junk() {
        assert!(answer("maybe\ny\n", false));
        assert!(!answer("what\nn\n", true));
    }

    fn no_spaces(text: &str) -> Result<()> {
        if text.contains(' ') {
            bail!("no spaces");
        }
        Ok(())
    }

    #[test]
    fn input_uses_default_on_empty_line_and_eof() {
        assert_eq!(input_from(&mut Cursor::new("
"), "?", "dflt", no_spaces), "dflt");
        assert_eq!(input_from(&mut Cursor::new(""), "?", "dflt", no_spaces), "dflt");
    }

    #[test]
    fn input_asks_again_until_valid() {
        let mut typed = Cursor::new("bad name
  good 
");
        assert_eq!(input_from(&mut typed, "?", "dflt", no_spaces), "good");
    }
}
