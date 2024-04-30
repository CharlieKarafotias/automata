mod nfa;
use nfa::{Nfa, Symbol};
fn main() {
    let nfa = Nfa::new(
        vec!["s0", "s1", "s2"],
        vec![Symbol::Value(0), Symbol::Value(1)],
        vec![
            ("s0", Symbol::Epsilon, vec!["s1"]),
            ("s1", Symbol::Epsilon, vec!["s2"]),
        ],
        "s0",
        vec!["s2"],
    );

    nfa.print();

    println!(
        "The input {} is accepted? {}",
        "",
        nfa.accept(vec![Symbol::Epsilon])
    );
}
