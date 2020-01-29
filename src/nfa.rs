fn step_nfa(current_state: &Vec<(char, Vec<usize>)>, current_char: char) -> (Vec<usize>, bool)
{
    for state in current_state {
        if state.0 == current_char {
            return (state.1.clone(), false);
        }
    }
    //ERROR could not find char 
    return (vec![0], true);
}

pub fn run(states: Vec<Vec<(char, Vec<usize>)>>, accepted: (usize, Vec<usize>), input_string: &str) -> bool
{
    let mut current_states = vec![accepted.0];
    for c in input_string.chars() {
        let mut new_state: Vec<usize> = vec![];
        for next in current_states {
            if next >= states.len() {
                print!("State Machine is failed. It should not be executet!");
                return false;
            }
            let (next_states, corrupt) = step_nfa(&states[next], c);
            if !corrupt {
                new_state.extend(next_states.iter().clone());
            }
        }
        current_states = new_state.clone();
    }
    for acc in accepted.1 {
        if current_states.contains(&acc) {
            return true;
        }
    }
    return false;
}
