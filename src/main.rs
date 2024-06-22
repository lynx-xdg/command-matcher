use std::fmt::Write;

// stolen from https://github.com/wooorm/levenshtein-rs/blob/main/src/lib.rs
/*
(The MIT License)

Copyright (c) 2016 Titus Wormer <tituswormer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
'Software'), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
fn levenshtein(a: &str, b: &str) -> usize {
    let mut result = 0;

    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result;
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();

    if length_a == 0 {
        return length_b;
    }

    if length_b == 0 {
        return length_a;
    }

    /* Initialize the vector.
     *
     * This is why it’s fast, normally a matrix is used,
     * here we use a single vector. */
    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let mut distance_a;
    let mut distance_b;

    /* Loop. */
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b {
                distance_a
            } else {
                distance_a + 1
            };

            distance_a = cache[index_a];

            result = if distance_a > result {
                if distance_b > result {
                    result + 1
                } else {
                    distance_b
                }
            } else if distance_b > distance_a {
                distance_a + 1
            } else {
                distance_b
            };

            cache[index_a] = result;
        }
    }

    result
}
// (end of MIT licensed code)

pub trait Command {
    fn matches(&self, str: &str) -> bool;
    fn matches_partial(&self, str: &str) -> Option<bool> { None }
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
