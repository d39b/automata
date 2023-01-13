use crate::rng;
use crate::automaton::{Automaton, neighbors};
use crate::image::ToNum;

#[derive(Clone)]
pub struct Cell {
    t: u64,
    score: i64,
}

impl ToNum for Cell {
    fn to_num(&self) -> u64 {
        self.t
    }
}

impl Cell {
    fn compute_score(&self, rng: &mut rng::UniformRng) -> i64 {
        return self.score + rng.sample(-5, 5);
    }
}

pub fn new_multi_type_automaton(n: usize, m: usize) -> Automaton<Cell, ()> {
    Automaton::new(n, m, init_random, elem_next_fn)
}

fn init_random(n: usize, m: usize) -> (Vec<Vec<Cell>>, ()) {
    let mut initial_grid = Vec::new();
    let mut rng = rng::UniformRng::new();
    for _ in 0..n {
        let mut row = Vec::new();
        for _ in 0..m {
            row.push(Cell {
                t: rng.sample(0, 10) as u64,
                score: rng.sample(-5, 5),
            });
        }
        initial_grid.push(row);
    }
    return (initial_grid, ());
}

pub fn elem_next_fn(x: &mut Vec<Vec<Cell>>, _: &mut ()) {
    const N_TYPES: usize = 10;
    let n = x.len();
    let m = x[0].len();

    let mut next = vec![vec![Cell{t: 0, score: 0}; m]; n];
    let mut rng = rng::UniformRng::new();

    let mut max_score = [-(1i64<<30); N_TYPES];
    for i in 0..n {
        for j in 0..m {
            let t = x[i][j].t as usize;
            let score = x[i][j].compute_score(&mut rng);
            if score > max_score[t] {
                max_score[t] = score;
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            let mut count = [0u64; N_TYPES];
            let nbs = neighbors(i, j, x, 4);
            for nb in nbs {
                let t = nb.t as usize;
                count[t] += 1;
            }

            // find most common type among neighbors
            let mut max = 0;
            let mut max_c = 0;
            for i in 0..N_TYPES {
                if count[i] > max {
                    max = count[i];
                    max_c = i;
                } else if count[i] == max {
                    if rng.sample(0, 2) == 0 {
                        max_c = i;
                    }
                }
            }

            // compare score of current cell with max score of cells
            // in most common group among neighbors
            if max_score[max_c] > x[i][j].compute_score(&mut rng) {
                next[i][j] = Cell{ t: max_c as u64, ..x[i][j] };
            } else {
                next[i][j] = x[i][j].clone();
            }
        }
    }
    *x = next;
}