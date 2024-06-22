use levenshtein::levenshtein;

pub trait Command {
    fn matches(&self, str: &str) -> bool;
    fn matches_partial(&self, _: &str) -> Option<bool> { None }
    fn distance(&self, str: &str) -> usize {
        levenshtein(self.to_str(), str)
    }
    fn to_str(&self) -> &str;
}

struct Echo {}

impl Command for Echo {
    fn matches(&self, str: &str) -> bool {
        str.starts_with("echo")
    }
    fn to_str(&self) -> &str {
        "Echo"
    }
}

struct Fimsh {}

impl Command for Fimsh {
    fn matches(&self, str: &str) -> bool {
        str == "fimsh"
    }
    fn to_str(&self) -> &str {
        "fimsh"
    }
}

enum Match<'v> {
    Excact(&'v Box<&'v dyn Command>),
    Close(&'v Box<&'v dyn Command>, usize)
}

impl std::fmt::Debug for Match<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Excact(cmd) => {
                write!(f, "Exact<{}>", cmd.to_str())?
            }
            Self::Close(cmd, distance) => {
                write!(f, "Close<{}, d={}>", cmd.to_str(), distance)?
            }
        };
        std::fmt::Result::Ok(())
    }
}

struct CommandMatcher<'a> {
    commands: Vec<Box<&'a dyn Command>>
}

impl<'a> CommandMatcher<'a> {
    fn register(&mut self, command: &'a (dyn Command + 'static))
    where
    {
        self.commands.push(Box::new(command));
    }
    fn find_match(&'a self, str: &str) -> Match{
        let mut best_match = &self.commands[0];
        let mut best_distance = best_match.distance(str);
        for command in &self.commands {
            if command.matches(str) {
                return Match::Excact(command);
            }
            let distance = command.distance(str);
            if distance < best_distance {
                best_distance = distance;
                best_match = command;
            }
        }
        Match::Close(best_match, best_distance)
    }
}

fn main() {
    println!("Hello, world!");
    let mut commands = CommandMatcher {
        commands: vec![]
    };
    commands.register(&Echo {});
    commands.register(&Fimsh {});

    let tests = vec!["fi", "echo", "µqs$dfq", "fimsh"];
    for test in tests {
        println!("{} => {:?}", test, commands.find_match(test));
    }
}
