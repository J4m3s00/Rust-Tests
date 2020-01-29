mod dfa;
mod nfa;


fn main() {

    //DFA
    ////////////////        q0                      q1
    let nodes = vec![vec![('0', 0), ('1', 4)], vec![('0', 0), ('1', 1)]];
    let accepted = (0, vec![1]);
    let input = "001011";

    print!("Result of input {} was {}!\n", input, dfa::run(nodes, accepted, &input));


    ///////NFA
    //                  char to read | states to go
    let nfa_states = vec![  vec![('0', vec![0]), ('1', vec![0, 1])],    //Q1
                            vec![('0', vec![2]), ('1', vec![2])],       //Q2
                            vec![]];                                    //Q3
    ///////////////Start   End
    
    //       start state | end states
    let nfa_accepted = (0, vec![2]);
    let nfa_input = "10";

    print!("Result of NFA with input \"{}\" was {}!\n", nfa_input, nfa::run(nfa_states, nfa_accepted, &nfa_input));
}