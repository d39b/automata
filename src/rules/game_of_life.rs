use crate::automaton::{Automaton, neighbors};
use crate::rng;

pub fn new_gol_automaton(n: usize, m: usize) -> Automaton<u8, ()> {
    Automaton::new(n, m, init_random, rule_next_fn)
}

fn init_random(n: usize, m: usize) -> (Vec<Vec<u8>>, ()) {
    let mut initial_grid = vec![vec![0; m]; n];
    let mut rng = rng::UniformRng::new();
	for i in 1..n-1 {
		for j in 1..m-1 {
        	if rng.sample(0, 100) < 2 {
        	    initial_grid[i][j] = 1
        	}
		}
    }
    return (initial_grid, ());
}

fn rule_next_fn(x: &mut Vec<Vec<u8>>, _: &mut ()) {
    let n = x.len();
    let m = x[0].len();

	let mut next_grid = vec![vec![0; m]; n];

	for i in 0..n {
    	for j in 0..m {
			let mut live_neighbours = 0;
			for nb in neighbors(i, j, &x, 1) {
				if *nb == 1 {
					live_neighbours += 1;
				}
			}
			if x[i][j] == 1 {
				if live_neighbours == 2 || live_neighbours == 3 {
					next_grid[i][j] = 1;
				}
			} else if live_neighbours == 3 {
				next_grid[i][j] = 1
			}
	    }
	}

    *x = next_grid
}