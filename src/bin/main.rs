use dtm::*;

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    Even,
    Odd,
    Reject,
    Accept,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Sigma {
    Zero,
    One,
}

fn delta(state: State, symbol: Option<Sigma>) -> (State, Option<Sigma>, Direction) {
    use Direction::*;
    use Sigma::*;
    use State::*;
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

#[test]
fn delta_test() {
    //10100 -> Accept
    use Direction::*;
    use Sigma::*;
    use State::*;
    assert_eq!(delta(Even, Some(One)), (Even, Some(One), Right));
    assert_eq!(delta(Even, Some(Zero)), (Odd, Some(Zero), Right));
    assert_eq!(delta(Odd, Some(One)), (Odd, Some(One), Right));
    assert_eq!(delta(Odd, Some(Zero)), (Even, Some(Zero), Right));
    assert_eq!(delta(Even, Some(Zero)), (Odd, Some(Zero), Right));
    assert_eq!(delta(Odd, None), (Accept, None, Stand));
    assert_eq!(delta(Accept, None), (Accept, None, Stand));
}

fn main() {
    let dtm = DTM::new(State::Even, State::Accept, State::Reject, delta);

    println!("{:?}", dtm);

    let input = {
        use Sigma::*;
        vec![One, Zero, One, Zero, Zero]
    };
    let (history, tape) = dtm.run(input);

    println!("History:\n{:?}", history);
    println!("Tape:\n{:?}", tape);

    let input = {
        use Sigma::*;
        vec![One, Zero, One, Zero]
    };

    let (history, tape) = dtm.run(input);

    println!("History:\n{:?}", history);
    println!("Tape:\n{:?}", tape);
}
