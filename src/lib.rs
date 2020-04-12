mod tape;

pub use tape::Tape;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Stand,
    Right,
}

#[derive(Debug)]
pub struct DTM<Q, S> {
    initial_state: Q,
    accepting_state: Q,
    rejecting_state: Q,
    delta: fn(Q, Option<S>) -> (Q, Option<S>, Direction),
}

impl<Q, S> DTM<Q, S>
where
    Q: PartialEq + Copy + Clone + std::fmt::Debug,
    S: PartialEq + Copy + std::fmt::Debug,
{
    pub fn new(
        initial_state: Q,
        accepting_state: Q,
        rejecting_state: Q,
        delta: fn(Q, Option<S>) -> (Q, Option<S>, Direction),
    ) -> DTM<Q, S> {
        DTM {
            initial_state,
            accepting_state,
            rejecting_state,
            delta,
        }
    }

    pub fn run(&self, input: Vec<S>) -> (Vec<Q>, Tape<S>) {
        let mut current_state = self.initial_state;
        let mut history: Vec<Q> = Vec::new();
        history.push(current_state);
        let mut tape = Tape::from(input);
        while self.accepting_state != current_state && self.rejecting_state != current_state {
            let (next_state, new_value, direction) = (self.delta)(current_state, tape.read());
            current_state = next_state;
            tape.write_and_move(new_value, direction);

            history.push(current_state);
        }

        (history, tape)
    }
}
