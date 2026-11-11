use std::io::{self, BufRead, IsTerminal, Write};

/// Asks a yes/no question. Without a terminal to answer on, it returns `default` instead of hanging
pub fn confirm(question: &str, default: bool) -> bool {
    if !io::stdin().is_terminal() {
        return default;
    }

    confirm_from(&mut io::stdin().lock(), question, default)
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
    use super::confirm_from;
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
}
