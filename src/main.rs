mod nfa;
use nfa::Nfa;
fn main() {
    let nfa = Nfa::new(
        vec!["s0", "s1", "s2"],
        vec!["0", "1"],
        vec![("s0", "", vec!["s1"]), ("s1", "", vec!["s2"])],
        "s0",
        vec!["s2"],
    );

    nfa.print();

    let input = "";
    println!("The input {} is accepted? {}", input, nfa.accept(input));
}
