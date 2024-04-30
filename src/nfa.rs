use std::fmt::Debug;

#[derive(Debug)]
pub struct Nfa<Q, S>
where
    Q: Debug + PartialEq + Ord + Clone,
    S: Debug + PartialEq,
{
    states: Vec<Q>,
    input_symbols: Vec<Symbol<S>>,
    transitions: Vec<(Q, Symbol<S>, Vec<Q>)>,
    initial_state: Q,
    final_states: Vec<Q>,
}

#[derive(Debug, PartialEq)]
pub enum Symbol<S> {
    Epsilon,
    Value(S),
}

impl<Q, S> Nfa<Q, S>
where
    Q: Debug + PartialEq + Ord + Clone,
    S: Debug + PartialEq,
{
    pub fn new(
        states: Vec<Q>,
        input_symbols: Vec<Symbol<S>>,
        transitions: Vec<(Q, Symbol<S>, Vec<Q>)>,
        initial_state: Q,
        final_states: Vec<Q>,
    ) -> Self {
        // Build validator here, one part should be turning "" to Epsilon
        Self {
            states,
            input_symbols,
            transitions,
            initial_state,
            final_states,
        }
    }

    pub fn print(&self) {
        println!("{self:?}");
    }

    pub fn possible_transitions(&self, state: &Q, input: Symbol<S>) -> Vec<Q> {
        self.transitions
            .iter()
            .filter(|x| x.0 == *state && x.1 == input)
            .flat_map(|x| x.2.clone())
            .collect()
    }

    pub fn e_closure<'b>(&self, state: Q) -> Vec<Q> {
        // Add initial state to return
        let mut closure = vec![state];
        let mut prev_len = 0;
        let mut new_len = closure.len();

        while prev_len != new_len {
            prev_len = new_len;
            let found: Vec<Q> = closure
                .iter()
                .flat_map(|state| self.possible_transitions(state, Symbol::Epsilon))
                .collect();
            closure.extend(found);
            closure.sort();
            closure.dedup();
            new_len = closure.len();
        }
        closure
    }

    pub fn accept(&self, input: Vec<Symbol<S>>) -> bool {
        let mut branches: Vec<Q> = vec![self.initial_state.clone()];
        branches.extend(self.e_closure(self.initial_state));
        branches.sort();
        branches.dedup();

        for i in 0..input.len() {
            let mut epsilons: Vec<Q> = vec![];
            branches
                .iter()
                .for_each(|state| epsilons.extend(self.e_closure(*state)));

            branches.extend(epsilons);
            branches.sort();
            branches.dedup();

            let mut new_branches: Vec<Q> = vec![];
            let char_slice: &[Symbol<S>] = input.get(i..i + 1).unwrap();
            branches.iter().for_each(|state| {
                new_branches.extend(self.possible_transitions(state, char_slice[0]))
            });

            new_branches.sort();
            new_branches.dedup();
            branches = new_branches;
        }
        branches.iter().any(|x| self.final_states.contains(x))
    }
}

mod tests {
    use super::{Nfa, Symbol};

    #[test]
    fn test_nfa_2_states() {
        let nfa = Nfa::new(
            vec!["p".to_string(), "q".to_string()],
            vec![Symbol::Value(0), Symbol::Value(0)],
            vec![
                ("p".to_string(), Symbol::Value(0), vec!["p".to_string()]),
                (
                    "p".to_string(),
                    Symbol::Value(1),
                    vec!["p".to_string(), "q".to_string()],
                ),
                ("q".to_string(), Symbol::Value(0), vec![]),
                ("q".to_string(), Symbol::Value(1), vec![]),
            ],
            "p".to_string(),
            vec!["q".to_string()],
        );

        // let nfa = Nfa {
        //     states: vec!["p", "q"],
        //     input_symbols: vec!["0", "1"],
        //     transitions: vec![
        //         ("p", "0", vec!["p"]),
        //         ("p", "1", vec!["p", "q"]),
        //         ("q", "0", vec![]),
        //         ("q", "1", vec![]),
        //     ],
        //     initial_state: "p",
        //     final_states: vec!["q"],
        // };
        assert!(nfa.accept(vec![Symbol::Value(1)]));
        assert!(nfa.accept(vec![Symbol::Value(1), Symbol::Value(1)]));
        assert!(!nfa.accept(vec![
            Symbol::Value(1),
            Symbol::Value(0),
            Symbol::Value(1),
            Symbol::Value(1)
        ]));
        // 10110000000000
        assert!(!nfa.accept(vec![
            Symbol::Value(1),
            Symbol::Value(0),
            Symbol::Value(1),
            Symbol::Value(1),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
            Symbol::Value(0),
        ]));
    }

    // #[test]
    // fn test_nfa_string_start_ab() {
    //     let nfa = Nfa {
    //         states: vec!["x", "y", "z"],
    //         input_symbols: vec!["a", "b"],
    //         transitions: vec![
    //             ("x", "a", vec!["y"]),
    //             ("y", "b", vec!["z"]),
    //             ("z", "a", vec!["z"]),
    //             ("z", "b", vec!["z"]),
    //         ],
    //         initial_state: "x",
    //         final_states: vec!["z"],
    //     };
    //     assert!(nfa.accept("ab"));
    //     assert!(nfa.accept("abba"));
    //     assert!(nfa.accept("abaa"));
    //     assert!(!nfa.accept("ba"));
    //     assert!(!nfa.accept("baa"));
    //     assert!(!nfa.accept("babaaa"));
    // }

    // #[test]
    // fn test_nfa_end_in_011_with_preceeding_011_or_end_in_101_with_preceeding_100() {
    //     let nfa = Nfa {
    //         states: vec![
    //             "q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10", "q11", "q12",
    //         ],
    //         input_symbols: vec!["0", "1"],
    //         transitions: vec![
    //             ("q0", "0", vec!["q0", "q1"]),
    //             ("q0", "1", vec!["q0", "q7"]),
    //             ("q1", "1", vec!["q2"]),
    //             ("q2", "1", vec!["q3"]),
    //             ("q3", "0", vec!["q3", "q4"]),
    //             ("q3", "1", vec!["q3"]),
    //             ("q4", "1", vec!["q5"]),
    //             ("q5", "0", vec!["q6"]),
    //             ("q7", "0", vec!["q8"]),
    //             ("q8", "0", vec!["q9"]),
    //             ("q9", "0", vec!["q9"]),
    //             ("q9", "1", vec!["q9", "q10"]),
    //             ("q10", "0", vec!["q11"]),
    //             ("q11", "1", vec!["q12"]),
    //         ],
    //         initial_state: "q0",
    //         final_states: vec!["q6", "q12"],
    //     };
    //     assert!(nfa.accept("011010"));
    //     assert!(nfa.accept("100101"));
    //     assert!(nfa.accept("111111011000001010"));
    //     assert!(nfa.accept("1111110010101010100000101"));
    //     assert!(!nfa.accept("010001010101010101010101000"));
    //     assert!(!nfa.accept("01000110101010101010101010001"));
    // }

    // #[test]
    // fn test_nfa_epsilon() {
    //     // Diagram: https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton#/media/File:NFAexample.svg
    //     let nfa = Nfa {
    //         states: vec!["s0", "s1", "s2", "s3", "s4"],
    //         input_symbols: vec!["0", "1"],
    //         transitions: vec![
    //             ("s0", "", vec!["s1", "s3"]),
    //             ("s1", "0", vec!["s2"]),
    //             ("s1", "1", vec!["s1"]),
    //             ("s2", "0", vec!["s1"]),
    //             ("s2", "1", vec!["s2"]),
    //             ("s3", "0", vec!["s3"]),
    //             ("s3", "1", vec!["s4"]),
    //             ("s4", "0", vec!["s4"]),
    //             ("s4", "1", vec!["s3"]),
    //         ],
    //         initial_state: "s0",
    //         final_states: vec!["s1", "s3"],
    //     };
    //     assert!(nfa.accept(""));
    //     assert!(nfa.accept("11"));
    //     assert!(nfa.accept("00"));
    //     assert!(nfa.accept("000"));
    //     assert!(!nfa.accept("1110"));
    //     assert!(!nfa.accept("01111100"));
    // }

    // #[test]
    // fn test_nfa_epsilon_to_epsilon_to_final() {
    //     // Diagram: https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton#/media/File:NFAexample.svg
    //     let nfa = Nfa {
    //         states: vec!["s0", "s1", "s2"],
    //         input_symbols: vec!["0", "1"],
    //         transitions: vec![("s0", "", vec!["s1"]), ("s1", "", vec!["s2"])],
    //         initial_state: "s0",
    //         final_states: vec!["s2"],
    //     };
    //     assert!(nfa.accept(""));
    //     assert!(!nfa.accept("1"));
    // }

    // #[test]
    // fn test_nfa_epsilon_example() {
    //     // Regex is: (a|b)*ac
    //     // Diagram: https://i.stack.imgur.com/vFx0x.png
    //     let nfa = Nfa {
    //         states: vec!["1", "2", "3", "4", "5", "6", "7", "8"],
    //         input_symbols: vec!["a", "b", "c"],
    //         transitions: vec![
    //             ("1", "", vec!["2", "5"]),
    //             ("2", "a", vec!["3"]),
    //             ("3", "c", vec!["4"]),
    //             ("5", "", vec!["6", "7"]),
    //             ("6", "a", vec!["8"]),
    //             ("7", "b", vec!["8"]),
    //             ("8", "", vec!["1"]),
    //         ],
    //         initial_state: "1",
    //         final_states: vec!["4"],
    //     };
    //     assert!(nfa.accept("ac"));
    //     assert!(nfa.accept("aac"));
    //     assert!(nfa.accept("bac"));
    //     assert!(nfa.accept("abac"));
    //     assert!(!nfa.accept("cac"));
    //     assert!(!nfa.accept("abaca"));
    // }
}
