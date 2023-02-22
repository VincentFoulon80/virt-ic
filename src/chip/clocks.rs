use std::time::Duration;

use crate::{generate_chip, impl_listener, State};

use super::{ChipBuilder, ChipRunner, ChipType, ListenerStorage, Pin, PinId, PinType};

#[derive(Debug, Clone, Copy)]
pub enum ClockEvent {
    Tick { state: State },
}

/// A customizable simple clock
/// CLK: clock
/// ```
///        --------
///  CLK --|1    4|-- VCC
///  GND --|2    3|-- UNUSED
///        --------
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Clock {
    frequency: Duration,
    timer: Duration,
    active: bool,
    #[serde(skip)]
    listeners: ListenerStorage<Self, ClockEvent>,
    pub vcc: Pin,
    pub gnd: Pin,
    pub clk: Pin,
}

impl Clock {
    pub const VCC: PinId = 4;
    pub const GND: PinId = 2;
    pub const CLK: PinId = 1;

    pub fn with_frequency(mut self, mut hertz: f64) -> Self {
        if hertz < f64::EPSILON {
            hertz = f64::EPSILON;
        }
        self.frequency = Duration::from_nanos((500_000_000.0 * (1.0 / hertz)) as u64);
        self
    }
}

impl ChipBuilder<Clock> for Clock {
    fn build() -> Clock {
        Clock {
            frequency: Duration::from_secs(1),
            listeners: ListenerStorage::default(),
            timer: Duration::default(),
            active: false,
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            clk: Pin::from(PinType::Output),
        }
    }
}

impl From<Clock> for ChipType {
    fn from(value: Clock) -> Self {
        ChipType::Clock(value)
    }
}

generate_chip!(Clock, vcc: Clock::VCC, gnd: Clock::GND, clk: Clock::CLK);

impl_listener!(Clock: listeners, ClockEvent);

impl ChipRunner for Clock {
    fn run(&mut self, tick_duration: Duration) {
        if self.vcc.state.as_logic(1.0) == State::High {
            self.timer += tick_duration;
            while self.timer > self.frequency {
                self.timer -= self.frequency;
                self.active = !self.active;
            }
            self.clk.state = State::from(self.active);
            self.trigger_event(ClockEvent::Tick {
                state: self.clk.state,
            });
        } else {
            self.active = false;
            self.timer = Duration::default();
        }
    }
}
