use crate::automaton::Automaton;
use crate::rng;

pub fn new_rule30_automaton(n: usize) -> Automaton<u8, (usize, RuleFunction)> {
    Automaton::new(n, 2*n+1, |n, _| -> (Vec<Vec<u8>>, (usize, RuleFunction)) {
        let grid = init_middle(n, 2*n+1);
        return (grid, (1, rule30));
    }, rule_next_fn)
}

pub fn new_rule184_automaton(n: usize, m: usize) -> Automaton<u8, (usize, RuleFunction)> {
    Automaton::new(n, m, |n, m| -> (Vec<Vec<u8>>, (usize, RuleFunction)) {
        let grid = init_random(n, m);
        return (grid, (1, rule184));
    }, rule_next_fn)
}

fn init_random(n: usize, m: usize) -> Vec<Vec<u8>> {
    let mut initial_grid = vec![vec![0; m]; n];
    let mut rng = rng::UniformRng::new();
    for j in 0..m-1 {
        if rng.sample(0, 2) == 1 {
            initial_grid[0][j] = 1
        }
    }
    return initial_grid;
}

fn init_middle(n: usize, m: usize) -> Vec<Vec<u8>> {
    let mut initial_grid = vec![vec![0; m]; n];
    initial_grid[0][m/2] = 1;
    return initial_grid;
}

type RuleFunction = fn(u8, u8, u8) -> u64;

fn rule_next_fn(x: &mut Vec<Vec<u8>>, y: &mut (usize, RuleFunction)) {
    let n = x.len();
    let m = x[0].len();
    let i = y.0;
    let rf = y.1;
    if i >= n {
        return
    }

    for j in 1..m-1 {
        match rf(x[i-1][j-1], x[i-1][j], x[i-1][j+1]) {
            1 => x[i][j] = 1,
            _ => {},
        }
    }

    y.0 = i + 1;
}

fn rule30(x: u8, y: u8, z: u8) -> u64 {
    match (x, y, z) {
        (1,0,0) | (0,1,1) | (0,1,0) | (0,0,1) => 1,
        _ => 0,
    }
}

fn rule184(x: u8, y: u8, z: u8) -> u64 {
    match (x, y, z) {
        (1,1,1) | (1,0,1) | (1,0,0) | (0,1,1) => 1,
        _ => 0,
    }
}