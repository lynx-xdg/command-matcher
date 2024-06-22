use std::io::{stdout, Read, Write};

use cmd_match::*;
struct Echo;

impl Command for Echo {
    fn matches(&self, str: &str) -> bool {
        str.starts_with("echo")
    }
    fn to_str(&self) -> &str {
        "echo"
    }
    fn exec(&self, args: &str) {
        println!("{}", args.split_at(5).1)
    }
}

struct Fimpsh;

impl Command for Fimpsh {
    fn matches(&self, str: &str) -> bool {
        str == "fimpsh"
    }
    fn to_str(&self) -> &str {
        "fimpsh"
    }
    fn exec(&self, _: &str) {
        println!(" _______  _______  _______  ______  _______  _______ \n|    ___||_     _||   |   ||   __ \\|     __||   |   |\n|    ___| _|   |_ |       ||    __/|__     ||       |\n|___|    |_______||__|_|__||___|   |_______||___|___|");
    }
}

fn read_line() -> String {
    use std::io::stdin;
    let mut buffer = String::new();
    let stdin = stdin();
    stdout().flush().unwrap();
    stdin.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    let mut commands = CommandMatcher::new();
    commands.register(&Echo);
    commands.register(&Fimpsh);

    loop {
        print!("> ");
        let test = &read_line();
        match commands.find_match(test, 3) {
            MatchResult::Hit(c) => c.exec(test),
            MatchResult::Near(c, d) => println!("did you mean {}? (distance={})", c.to_str(), d),
            MatchResult::Miss => println!("Command '{}' not found", test), 
        }
    }
}
