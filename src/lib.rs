#[derive(Debug)]
pub enum Direction {
    Left,
    None,
    Right,
}

#[derive(Debug)]
pub struct DTM<Q, S> {
    initial_state: Q,
    accepting_state: Q,
    refusing_state: Q,
    delta: fn(Q, Option<S>) -> (Q, Option<S>, Direction),
}

#[derive(Debug)]
pub enum DTMError {
    AcceptEqualRefuse,
}

impl<Q, S> DTM<Q, S>
where
    Q: PartialEq,
{
    pub fn new(
        initial_state: Q,
        accepting_state: Q,
        refusing_state: Q,
        delta: fn(Q, Option<S>) -> (Q, Option<S>, Direction),
    ) -> Result<DTM<Q, S>, DTMError> {
        if initial_state == refusing_state {
            Err(DTMError::AcceptEqualRefuse)
        } else {
            Ok(DTM {
                initial_state,
                accepting_state,
                refusing_state,
                delta,
            })
        }
    }
}
