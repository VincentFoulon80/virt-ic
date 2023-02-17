use std::time::{Duration, Instant};

use crate::{
    chip::{Chip, PinId, PinType},
    utilities::{Id, Storage},
    State,
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Board<C: Chip> {
    chips: Storage<C>,
    traces: Storage<Trace<C>>,
}

impl<C> Board<C>
where
    C: Chip,
{
    pub fn new() -> Self {
        Board {
            chips: Storage::default(),
            traces: Storage::default(),
        }
    }

    pub fn run(&mut self, tick_duration: Duration) {
        for (_id, chip) in self.chips.as_mut_vec() {
            let mut pins_to_reset = vec![];
            for (pin_id, pin) in chip.list_pins() {
                if matches!(pin.pin_type, PinType::Input) {
                    pins_to_reset.push(pin_id);
                }
            }
            for pin_id in pins_to_reset {
                if let Some(pin) = chip.get_pin_mut(pin_id) {
                    pin.state = State::Undefined
                }
            }
        }

        for (_id, trace) in self.traces.as_mut_vec() {
            trace.calculate_state(&mut self.chips);
        }

        for (_id, chip) in self.chips.as_mut_vec() {
            chip.run(tick_duration);
        }
    }

    /// Run the circuit for a certain amount of time segmented by a step
    /// The smaller the step the more accurate the simulation will be.
    pub fn run_during(&mut self, duration: Duration, step: Duration) {
        let mut elapsed = Duration::default();
        while elapsed < duration {
            self.run(step);
            elapsed += step;
        }
    }

    pub fn run_realtime(&mut self, duration: Duration) {
        let instant = Instant::now();
        let mut old = Instant::now();
        let mut new = Instant::now();
        while instant.elapsed() <= duration {
            self.run(new.duration_since(old));
            old = new;
            new = Instant::now();
        }
    }

    pub fn register_chip(&mut self, chip: C) -> Id<C> {
        self.chips.add(chip)
    }

    pub fn register_trace(&mut self, trace: Trace<C>) -> Id<Trace<C>> {
        self.traces.add(trace)
    }

    pub fn connect(
        &mut self,
        chip_a: Id<C>,
        pin_a: PinId,
        chip_b: Id<C>,
        pin_b: PinId,
    ) -> Id<Trace<C>> {
        self.traces.add(Trace {
            pins: vec![(chip_a, pin_a), (chip_b, pin_b)],
        })
    }

    pub fn get_chip(&self, id: &Id<C>) -> &C {
        self.chips.get(id)
    }

    pub fn get_chip_mut(&mut self, id: &Id<C>) -> &mut C {
        self.chips.get_mut(id)
    }

    pub fn get_trace(&self, id: &Id<Trace<C>>) -> &Trace<C> {
        self.traces.get(id)
    }

    pub fn get_trace_mut(&mut self, id: &Id<Trace<C>>) -> &mut Trace<C> {
        self.traces.get_mut(id)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trace<C: Chip> {
    pins: Vec<(Id<C>, usize)>,
}

impl<C> Trace<C>
where
    C: Chip,
{
    pub fn new() -> Self {
        Trace { pins: Vec::new() }
    }

    pub fn connect(&mut self, chip: Id<C>, pin: PinId) {
        if !self.pins.contains(&(chip, pin)) {
            self.pins.push((chip, pin))
        }
    }

    pub fn disconnect(&mut self, chip: Id<C>, pin: PinId) {
        self.pins.retain(|&x| x != (chip, pin));
    }

    pub fn get_connections(&self) -> &[(Id<C>, usize)] {
        &self.pins
    }

    pub fn calculate_state(&mut self, chip_storage: &mut Storage<C>) {
        let mut base_state = State::Undefined;
        // read state
        for (chip_id, pin_id) in self.pins.iter() {
            let chip = chip_storage.get(chip_id);
            if let Some(pin) = chip.get_pin(*pin_id) {
                if matches!(pin.pin_type, PinType::Output) {
                    base_state = base_state.feed_state(pin.state);
                }
            }
        }
        // write state
        for (chip_id, pin_id) in self.pins.iter() {
            let chip = chip_storage.get_mut(chip_id);
            if let Some(mut pin) = chip.get_pin_mut(*pin_id) {
                if matches!(pin.pin_type, PinType::Input) {
                    pin.state = pin.state.feed_state(base_state);
                }
            }
        }
    }
}
