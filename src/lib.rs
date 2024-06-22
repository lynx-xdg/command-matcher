#[cfg(feature="levenshtein")]
use levenshtein::levenshtein;

pub trait Command {
    fn matches(&self, str: &str) -> bool;
    #[cfg(feature="levenshtein")]
    fn distance(&self, str: &str) -> usize {
        levenshtein(self.to_str(), str)
    }
    #[cfg(all(not(feature="levenshtein"), feature="distance"))]
    fn distance(&self, _: &str) -> usize {
        println!("distance not implemented for {}", self.to_str());
        usize::MAX
    }
    fn to_str(&self) -> &str;
    fn exec(&self, args: &str);
}

pub enum MatchResult<'a> {
    Hit(&'a dyn Command),
    #[cfg(feature="distance")]
    Near(&'a dyn Command, usize),
    Miss
}
impl std::fmt::Debug for MatchResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchResult::Hit(c) => write!(f, "Hit({:?})", c.to_str()),
            MatchResult::Near(c, d) => write!(f, "Near({:?}, {})", c.to_str(), d),
            MatchResult::Miss => write!(f, "Miss")
        }
    }
}

pub struct CommandMatcher<'a> {
    commands: Vec<&'a dyn Command>
}

impl<'a> CommandMatcher<'a> {
    pub fn register(&mut self, command: &'a dyn Command) {
        self.commands.push(command);
    }
    #[cfg(feature="distance")]
    pub fn find_match(&'a self, str: &str, max_distance: usize) -> MatchResult<'a> {
        let mut best_match = self.commands[0];
        let mut best_distance = best_match.distance(str);
        for command in &self.commands {
            if command.matches(str) {
                return MatchResult::Hit(*command);
            }
            let distance = command.distance(str);
            if distance < best_distance {
                best_match = *command;
                best_distance = distance;
            }
        }
        if best_distance <= max_distance {
            MatchResult::Near(best_match, best_distance)
        } else {
            MatchResult::Miss
        }
    }
    #[cfg(not(feature="distance"))]
    pub fn find_match(&'a self, str: &str) -> MatchResult<'a> {
        for command in &self.commands {
            if command.matches(str) {
                return MatchResult::Hit(command);
            }
        }
        MatchResult::Miss
    }
}

impl CommandMatcher<'_> {
    pub fn new() -> CommandMatcher<'static> {
        CommandMatcher {
            commands: Vec::new()
        }
    }
}