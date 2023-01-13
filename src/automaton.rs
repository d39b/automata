pub type TransitionFunction<T, S> = fn(&mut Vec<Vec<T>>, &mut S);
pub type InitFunction<T, S> = fn(usize, usize) -> (Vec<Vec<T>>, S);

// A cellular automaton, whose state consists of:
// * n by m grid of elements of type T
// * a "global" state value of type S (can be () if not required)
//
// The actual behavior of the automaton is defined by a transition function.
pub struct Automaton<T, S> {
    grid: Vec<Vec<T>>,
    global_state: S,
    init_fn: InitFunction<T, S>,
    next_fn: TransitionFunction<T, S>,
}

impl<T, S> Automaton<T, S> {
    // Creates a new automaton with the given grid size, init and transition function.
    // The transition function computes the next state of the automaton given mutable references to the
    // grid and global state.
    pub fn new(n: usize, m: usize, init_fn: InitFunction<T, S>, next_fn: TransitionFunction<T, S>) -> Automaton<T, S> {
        let (grid, global_state) = init_fn(n, m);
        Automaton {
            grid: grid,
            global_state: global_state,
            init_fn: init_fn,
            next_fn: next_fn,
        }
    }
    
    // Returns (height, width) of the grid
    pub fn size(&self) -> (usize, usize) {
        let n = self.grid.len();
        if n == 0 {
            return (0, 0);
        }
        return (n, self.grid[0].len())
    }

    // Returns the current grid and global state
    pub fn state(&self) -> (&Vec<Vec<T>>, &S) {
        (&self.grid, &self.global_state)
    }

    // Update the state by calling the transition function
    pub fn next(&mut self) {
        (self.next_fn)(&mut self.grid, &mut self.global_state);
    }

    pub fn reset(&mut self, n: usize, m: usize) {
        let (grid, global_state) = (self.init_fn)(n, m);
        self.grid = grid;
        self.global_state = global_state;
    }
}

// Returns all the cells in a (2k+1) square grid centered at the cell (x,y).
pub fn neighbors<T>(x: usize, y: usize, v: &Vec<Vec<T>>, k: usize) -> Vec<&T> {
    let mut result = Vec::new();

    if v.len() == 0 {
        return result;
    }

    let n = v.len();
    let m = v[0].len();

    for i in 0..k+1 {
        for j in 0..k+1 {
            if i == 0 && j == 0 {
                continue;
            }
            if x >= i && y >= j {
                result.push(&v[x-i][y-j]);
            }
            if x+i < n && y+j < m {
                result.push(&v[x+i][y+j]);
            }
            if x >= i && y+j < m {
                result.push(&v[x-i][y+j]);
            }
            if x+i < n && y >= j {
                result.push(&v[x+i][y-j]);
            }
        }
    }
    return result;
}
