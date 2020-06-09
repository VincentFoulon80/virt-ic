use super::{Trace, Socket, Chip};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// A Board that contains Traces and Sockets
#[derive(Default)]
pub struct Board {
    traces: Vec<Rc<RefCell<Trace>>>,
    sockets: Vec<Rc<RefCell<Socket>>>
}

impl Board {
    /// Create a new empty Board
    pub fn new() -> Board {
        Board {
            traces: vec![],
            sockets: vec![]
        }
    }

    /// Create a new trace and return it
    pub fn new_trace(&mut self) -> Rc<RefCell<Trace>> {
        let trace = Rc::new(RefCell::new(Trace::new()));
        self.traces.push(trace);
        // unwrap because we just pushed a value so there's no reason to get a None here
        self.traces.last_mut().unwrap().clone()
    }

    /// Create a new socket and return it
    /// Note that you'll have to plug a chip on it before linking it with the traces
    pub fn new_socket(&mut self) -> Rc<RefCell<Socket>> {
        let socket = Rc::new(RefCell::new(Socket::new()));
        self.sockets.push(socket);
        // unwrap because we just pushed a value so there's no reason to get a None here
        self.sockets.last_mut().unwrap().clone()
    }

    /// Create a new socket with a chip and return it
    pub fn new_socket_with(&mut self, chip: Box<dyn Chip>) -> Rc<RefCell<Socket>> {
        let socket = Rc::new(RefCell::new(Socket::new()));
        socket.borrow_mut().plug(chip);
        self.sockets.push(socket);
        // unwrap because we just pushed a value so there's no reason to get a None here
        self.sockets.last_mut().unwrap().clone()
    }

    /// Run the circuit for a certain amount of time
    /// You must use `use_during` since it provides more accurate simulation by stepping
    pub fn run(&mut self, time_elapsed : Duration) {
        // TODO: find a way to update the traces accurately
        // current issue : the order of the traces affects the order of the links
        for trc in self.traces.iter_mut() {
            trc.borrow_mut().communicate();
        }
        for skt in self.sockets.iter_mut() {
            skt.borrow_mut().run(time_elapsed);
        }
    }

    /// Run the circuit for a certain amount of time segmented by a step
    /// The smaller the step the more accurate the simulation will be.
    pub fn run_during(&mut self, duration: Duration, step: Duration) {
        let mut elapsed = Duration::new(0,0);
        while elapsed < duration {
            self.run(step);
            elapsed += step;
        }
    }

    pub fn run_realtime(&mut self, duration: Duration) {
        let instant = Instant::now();
        let mut old = Instant::now();
        let mut new = Instant::now();
        let mut count = 0;
        while instant.elapsed() <= duration {
            self.run(new.duration_since(old));
            old = new;
            new = Instant::now();
            count += 1;
        }
    }
}