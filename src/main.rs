use std::io::Error;
use std::io::Write;

mod example;

fn main() {
    print_message(&mut std::io::stdout()).unwrap();
}

fn print_message(stdout: &mut dyn Write) -> Result<(), Error> {
    stdout.write(format!("{}\n", example::message()).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdoutDouble {
        pub written_content: Option<String>,
    }

    impl StdoutDouble {
        fn new() -> StdoutDouble {
            StdoutDouble {
                written_content: None,
            }
        }
    }

    impl Write for StdoutDouble {
        fn write(&mut self, content: &[u8]) -> std::result::Result<usize, std::io::Error> {
            self.written_content = Some(std::str::from_utf8(content).unwrap().to_string());

            Ok(0)
        }
        fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[test]
    fn prints_message() {
        let mut stdout = StdoutDouble::new();

        print_message(&mut stdout).unwrap();

        assert_eq!("Rust Example\n", stdout.written_content.unwrap())
    }
}
