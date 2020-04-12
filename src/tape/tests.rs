use super::Tape;

fn standard_tape() -> Tape<isize> {
    Tape {
        positive: vec![0, 1, 2, 3].into_iter().map(Some).collect(),
        negative: vec![-1, -2, -3, -4].into_iter().map(Some).collect(),
        index: 0,
    }
}

#[cfg(test)]
mod read_tests {
    #[test]
    fn normal_behaviour() {
        let mut tape = super::standard_tape();
        for e in -4..=3 {
            tape.index = e;
            assert_eq!(Some(e), tape.read());
        }
    }

    #[test]
    fn out_of_bounds_behaviour() {
        let mut tape = super::standard_tape();
        let indices = vec![6, 5, -5, -6];
        for i in indices {
            tape.index = i;
            assert!(tape.read().is_none());
        }
    }
}

#[cfg(test)]
mod read_at_tests {
    #[test]
    fn normal_behaviour() {
        let tape = super::standard_tape();
        for e in -4..=3 {
            assert_eq!(Some(e), tape.read_at(e));
        }
    }

    #[test]
    fn out_of_bounds_behaviour() {
        let tape = super::standard_tape();
        assert_eq!(tape.read_at(3), Some(3));
        assert_eq!(tape.read_at(4), None);
        assert_eq!(tape.read_at(-4), Some(-4));
        assert_eq!(tape.read_at(-5), None);
    }
}
