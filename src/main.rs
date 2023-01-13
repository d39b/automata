use std::{thread, time};
use simulation::Simulation;
use image::{FlatImg, ToNum};
use crate::rules::{one_dim::new_rule30_automaton, one_dim::new_rule184_automaton ,game_of_life::new_gol_automaton, multi_type::new_multi_type_automaton};

mod automaton;
mod display;
mod rng;
mod image;
mod simulation;
mod rules;

fn main() {
    let n = 200;
    let m = 200;

    let aut = new_gol_automaton(n, m);
    // let aut = new_rule30_automaton(n);
    // let aut = new_rule184_automaton(n, m);
    // let aut = new_multi_type_automaton(n, m);

    let delay = time::Duration::from_millis(50);
    let (mut simulation, output_recv, command_send) = Simulation::new(
        aut,
        transform,
        delay,
    ); 

    thread::spawn(move|| {
        simulation.run();
    });
    display::run(output_recv, command_send);
}

fn transform<T: ToNum,S>(grid: &Vec<Vec<T>>, global_state: &S) -> FlatImg {
    FlatImg::from_2d_vec(grid)
}