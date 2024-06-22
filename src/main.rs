use cmd_match::*;
struct Echo;

impl Command for Echo {
    fn matches(&self, str: &str) -> bool {
        str.starts_with("echo")
    }
    fn to_str(&self) -> &str {
        "Echo"
    }
    fn exec(&self, args: &str) {
        println!("echo invoked from: {}", args);
    }
}

struct Fimsh;

impl Command for Fimsh {
    fn matches(&self, str: &str) -> bool {
        str == "fimsh"
    }
    fn to_str(&self) -> &str {
        "fimsh"
    }
    fn exec(&self, args: &str) {
        println!("fimsh invoked from: {}", args);
    }
}

fn main() {
    println!("Hello, world!");
    let mut commands = CommandMatcher::new();
    commands.register(&Echo);
    commands.register(&Fimsh);

    let tests = vec!["fi", "echo", "µqs$dfq", "fimsh"];
    for test in tests {
        println!("{} => {:?}", test, commands.find_match(test, 4));
    }
}
