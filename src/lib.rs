pub mod board;
pub mod chip;
pub mod utilities;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum State {
    #[default]
    Undefined,
    Low,
    High,
    Analog(f32),
}

impl State {
    pub fn feed_state(&mut self, state: State) -> Self {
        match state {
            State::Low if matches!(self, State::Undefined) => State::Low,
            State::High => State::High,
            State::Analog(_) if matches!(self, State::High) => State::High,
            State::Analog(v) => {
                if let State::Analog(bv) = self {
                    if v < *bv {
                        *self
                    } else {
                        State::Analog(v)
                    }
                } else {
                    State::Analog(v)
                }
            }
            State::Undefined | State::Low => *self,
        }
    }

    pub fn as_analog(&self, conversion_target: f32) -> Self {
        match self {
            State::Undefined | State::Low => State::Analog(0.0),
            State::High => Self::Analog(conversion_target),
            State::Analog(_) => *self,
        }
    }

    pub fn as_logic(&self, threshold: f32) -> Self {
        match self {
            State::Undefined => State::Low,
            State::Low | State::High => *self,
            State::Analog(v) => {
                if *v >= threshold {
                    State::High
                } else {
                    State::Low
                }
            }
        }
    }
}

impl From<State> for bool {
    fn from(value: State) -> Self {
        match value {
            State::Undefined | State::Low => false,
            State::High => true,
            State::Analog(v) => v != 0.0,
        }
    }
}

impl From<bool> for State {
    fn from(value: bool) -> Self {
        if value {
            State::High
        } else {
            State::Low
        }
    }
}
