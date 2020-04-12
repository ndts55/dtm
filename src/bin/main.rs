use dtm::*;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Q {
    Even,
    Odd,
    Reject,
    Accept,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum S {
    Zero,
    One,
}

fn delta(state: Q, symbol: Option<S>) -> (Q, Option<S>, Direction) {
    use Direction::*;
    use Q::*;
    use S::*;
    match (state, symbol) {
        (Reject, _) => (Reject, None, Stand),
        (Accept, _) => (Accept, None, Stand),
        (Even, Some(Zero)) => (Odd, Some(Zero), Right),
        (Odd, Some(Zero)) => (Even, Some(Zero), Right),
        (any, Some(One)) => (any, Some(One), Right),
        (Even, None) => (Reject, None, Stand),
        (Odd, None) => (Accept, None, Stand),
    }
}

fn main() {
    let dtm = DTM::new(Q::Even, Q::Accept, Q::Reject, delta);

    println!("{:?}", dtm);

    let input = {
        use S::*;
        vec![One, Zero, One, Zero, Zero]
    };
    let (history, tape) = dtm.run(input);

    println!("History:\n{:?}", history);
    println!("Tape:\n{:?}", tape);

    let input = {
        use S::*;
        vec![One, Zero, One, Zero]
    };

    let (history, tape) = dtm.run(input);

    println!("History:\n{:?}", history);
    println!("Tape:\n{:?}", tape);
}
