fn step_nfa(current_state: &Vec<(char, usize)>, current_char: char) -> (usize, bool)
{
    for s in current_state {
        if s.0 == current_char {
            return (s.1, false);
        }
    }
    //ERROR NOTHING TO GO TO. 
    return (0, true);
}

pub fn run(states: Vec<Vec<(char, usize)>>, accepted: (usize, Vec<usize>), input_text: &str) -> bool
{
    let mut current_state = accepted.0;
    for c in input_text.chars() { 
        let (next_state, corrupt) = step_nfa(&states[current_state], c);
        current_state = next_state;
        if corrupt { print!("No next state found! \nStopping NFA!\n"); return false; }
    }
    return accepted.1.contains(&current_state);
}