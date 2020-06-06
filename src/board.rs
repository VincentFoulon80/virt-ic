use super::{Trace, Socket, Chip};
use std::cell::RefCell;
use std::rc::Rc;

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
        self.traces.last_mut().unwrap().clone()
    }

    /// Create a new socket and return it
    pub fn new_socket(&mut self) -> Rc<RefCell<Socket>> {
        let socket = Rc::new(RefCell::new(Socket::new()));
        self.sockets.push(socket);
        self.sockets.last_mut().unwrap().clone()
    }

    /// Create a new socket with a chip and return it
    pub fn new_socket_with(&mut self, chip: Box<dyn Chip>) -> Rc<RefCell<Socket>> {
        let socket = Rc::new(RefCell::new(Socket::new()));
        socket.borrow_mut().plug(chip);
        self.sockets.push(socket);
        self.sockets.last_mut().unwrap().clone()
    }

    /// Run the circuit for a certain amount of time
    pub fn run(&mut self, time_elapsed : std::time::Duration) {
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
    pub fn run_during(&mut self, duration: std::time::Duration, step: std::time::Duration) {
        let mut elapsed = std::time::Duration::new(0,0);
        while elapsed < duration {
            self.run(step);
            elapsed += step;
        }
    }
}