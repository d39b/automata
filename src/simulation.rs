use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::{thread, time};
use std::option::Option;
use crate::automaton::Automaton;

// A Simulation can be used to run an Automaton in another thread.
//
// Two channels are used to interact with the simulation, an output channel and a command channel.
// After every step of the automaton, the current simulation state is sent on the output channel.
// Commands can be sent to the command channel to e.g. start/stop the simulation.
pub struct Simulation<T: Send, S: Send, U: Send> {
    aut: Automaton<T, S>,
    output_send: Sender<SimulationState<U>>,
    command_recv: Receiver<Command>,
    transform: TransformFunction<T, S, U>,
    // current step
    step: u64,
    // true if the simulation is currently running
    running: bool,
    // how long to sleep after each step
    delay: time::Duration,
}

pub enum Command {
    Start,
    Stop,
    SingleStep,
    ChangeDelay(std::time::Duration),
    Reset(usize, usize),
}

pub struct SimulationState<U> {
    pub data: U,
    pub step: u64,
    pub running: bool,
}

pub type TransformFunction<T, S, U> = fn(&Vec<Vec<T>>, &S) -> U;

impl<T: Send, S: Send, U: Send> Simulation<T, S, U> {
    pub fn new(aut: Automaton<T, S>, transform: TransformFunction<T, S, U>, delay: time::Duration) -> (Simulation<T, S, U>, Receiver<SimulationState<U>>, Sender<Command>) {
        let (output_send, output_recv) = channel();
        let (command_send, command_recv) = channel();
        (Simulation { 
            aut: aut,
            output_send: output_send,
            command_recv: command_recv,
            transform: transform,
            step: 0,
            running: false,
            delay: delay,
        }, output_recv, command_send)
    }

    fn send_state(&self) {
        let aut_state = self.aut.state();
        let transformed_state = (self.transform)(aut_state.0, aut_state.1);
        let simulation_state = SimulationState{
            data: transformed_state,
            step: self.step,
            running: self.running,
        };

        match self.output_send.send(simulation_state) {
            Err(_) => println!("could not send simulation state"),
            _ => {},
        }
    }

    fn recv_command(&mut self) {
        let command: Option<Command>;
        if self.running {
            command = match self.command_recv.try_recv() {
                Ok(c) => Option::Some(c),
                Err(e) => match e {
                    TryRecvError::Disconnected => {
                        println!("error command channel disconnected");
                        Option::None
                    }
                    TryRecvError::Empty => Option::None,
                },
            }
        } else {
            // If the simulation is not running currently, this call to read a command
            // from the command channel will block.
            command = match self.command_recv.recv() {
                Ok(c) => Option::Some(c),
                Err(_) => {
                    println!("error command channel disconnected");
                    Option::None
                }
            }
        }

        match command {
            Some(c) => match c {
                Command::Start => self.running = true,
                Command::Stop => self.running = false,
                Command::SingleStep => if !self.running { self.step(); },
                Command::ChangeDelay(d) => self.delay = d,
                Command::Reset(n, m) => {
                    self.running = false;
                    self.aut.reset(n, m);
                    self.send_state();
                },
            },
            _ => {},
        }
    }

    fn step(&mut self) {
        self.aut.next();
        self.step += 1;
        self.send_state();
    }

    pub fn run(&mut self) {
        // send the initial state
        self.send_state();
        loop {
            self.recv_command();
            if self.running {
                self.step();
                thread::sleep(self.delay);
            }
        }
    }
}