use std::collections::VecDeque;

#[derive(Debug)]
struct Nfa<'a> {
    states: Vec<&'a str>,
    input_symbols: Vec<&'a str>,
    transitions: Vec<(&'a str, &'a str, Vec<&'a str>)>,
    initial_state: &'a str,
    final_states: Vec<&'a str>,
}

impl<'a> Nfa<'a> {
    fn new(
        states: Vec<&'a str>,
        input_symbols: Vec<&'a str>,
        transitions: Vec<(&'a str, &'a str, Vec<&'a str>)>,
        initial_state: &'a str,
        final_states: Vec<&'a str>,
    ) -> Self {
        Self {
            states,
            input_symbols,
            transitions,
            initial_state,
            final_states,
        }
    }

    fn print(&self) {
        println!("{self:?}");
    }

    fn check_branch(&self, state: &str, input: char) -> Vec<&str> {
        let transition = self
            .transitions
            .iter()
            .find(|&x| x.0 == state && x.1 == input.to_string());
        if let Some(transition) = transition {
            transition.2.clone()
        } else {
            // panic!("Transition not found for state: {state}, input: {input}")
            vec![]
        }
    }

    fn accept(&self, input: &str) -> bool {
        // input to chars
        let mut chars: VecDeque<char> = input.chars().collect();
        let mut branches: Vec<&str> = vec![self.initial_state];
        while !chars.is_empty() {
            let char = chars.pop_front().unwrap();
            let mut new_branches: Vec<&str> = vec![];
            for state in branches {
                new_branches.extend(self.check_branch(state, char));
            }
            branches = new_branches;
            println!("Branches after processing char: {char}: {branches:?}");
        }
        branches.iter().any(|x| self.final_states.contains(x))
    }
}

fn main() {
    let nfa = Nfa {
        states: vec!["p", "q"],
        input_symbols: vec!["0", "1"],
        transitions: vec![
            ("p", "0", vec!["p"]),
            ("p", "1", vec!["p", "q"]),
            ("q", "0", vec![]),
            ("q", "1", vec![]),
        ],
        initial_state: "p",
        final_states: vec!["q"],
    };

    nfa.print();

    let input = "1011";
    println!("The input {} is accepted? {}", input, nfa.accept(input));
}

mod tests {
    use super::Nfa;

    #[test]
    fn test_nfa_2_states() {
        let nfa = Nfa {
            states: vec!["p", "q"],
            input_symbols: vec!["0", "1"],
            transitions: vec![
                ("p", "0", vec!["p"]),
                ("p", "1", vec!["p", "q"]),
                ("q", "0", vec![]),
                ("q", "1", vec![]),
            ],
            initial_state: "p",
            final_states: vec!["q"],
        };
        assert!(nfa.accept("1"));
        assert!(nfa.accept("11"));
        assert!(nfa.accept("1011"));
        assert!(!nfa.accept("10110000000000"));
    }

    #[test]
    fn test_nfa_string_start_ab() {
        let nfa = Nfa {
            states: vec!["x", "y", "z"],
            input_symbols: vec!["a", "b"],
            transitions: vec![
                ("x", "a", vec!["y"]),
                ("y", "b", vec!["z"]),
                ("z", "a", vec!["z"]),
                ("z", "b", vec!["z"]),
            ],
            initial_state: "x",
            final_states: vec!["z"],
        };
        assert!(nfa.accept("ab"));
        assert!(nfa.accept("abba"));
        assert!(nfa.accept("abaa"));
        assert!(!nfa.accept("ba"));
        assert!(!nfa.accept("baa"));
        assert!(!nfa.accept("babaaa"));
    }

    #[test]
    fn test_nfa_end_in_011_with_preceeding_011_or_end_in_101_with_preceeding_100() {
        let nfa = Nfa {
            states: vec![
                "q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10", "q11", "q12",
            ],
            input_symbols: vec!["0", "1"],
            transitions: vec![
                ("q0", "0", vec!["q0", "q1"]),
                ("q0", "1", vec!["q0", "q7"]),
                ("q1", "1", vec!["q2"]),
                ("q2", "1", vec!["q3"]),
                ("q3", "0", vec!["q3", "q4"]),
                ("q3", "1", vec!["q3"]),
                ("q4", "1", vec!["q5"]),
                ("q5", "0", vec!["q6"]),
                ("q7", "0", vec!["q8"]),
                ("q8", "0", vec!["q9"]),
                ("q9", "0", vec!["q9"]),
                ("q9", "1", vec!["q9", "q10"]),
                ("q10", "0", vec!["q11"]),
                ("q11", "1", vec!["q12"]),
            ],
            initial_state: "q0",
            final_states: vec!["q6", "q12"],
        };
        assert!(nfa.accept("011010"));
        assert!(nfa.accept("100101"));
        assert!(nfa.accept("111111011000001010"));
        assert!(nfa.accept("1111110010101010100000101"));
        assert!(!nfa.accept("010001010101010101010101000"));
        assert!(!nfa.accept("01000110101010101010101010001"));
    }
}
