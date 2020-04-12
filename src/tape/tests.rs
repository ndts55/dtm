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
mod write_tests {
    #[test]
    fn at_valid_index() {
        let mut tape = super::standard_tape();

        for (i, e) in (-4..=3).enumerate() {
            tape.index = e;
            tape.write(Some(i as isize));
        }

        assert_eq!(
            vec![3, 2, 1, 0]
                .into_iter()
                .map(Some)
                .collect::<Vec<Option<isize>>>(),
            tape.negative
        );

        assert_eq!(
            vec![4, 5, 6, 7]
                .into_iter()
                .map(Some)
                .collect::<Vec<Option<isize>>>(),
            tape.positive
        );
    }

    #[test]
    fn at_invalid_index() {
        let mut tape = super::standard_tape();

        tape.index = -5;
        tape.write(Some(-5));
        tape.index = 4;
        tape.write(Some(4));

        assert_eq!(
            vec![-1, -2, -3, -4, -5]
                .into_iter()
                .map(Some)
                .collect::<Vec<Option<isize>>>(),
            tape.negative
        );

        assert_eq!(
            vec![0, 1, 2, 3, 4]
                .into_iter()
                .map(Some)
                .collect::<Vec<Option<isize>>>(),
            tape.positive
        );
    }
}

#[cfg(test)]
mod shift_tests {
    use super::super::Direction::*;
    #[test]
    fn left() {
        let mut tape = super::standard_tape();
        assert_eq!(tape.index, 0);
        tape.shift(Left);
        assert_eq!(tape.index, -1);
    }

    #[test]
    fn right() {
        let mut tape = super::standard_tape();
        assert_eq!(tape.index, 0);
        tape.shift(Right);
        assert_eq!(tape.index, 1);
    }

    #[test]
    fn stand() {
        let mut tape = super::standard_tape();
        assert_eq!(tape.index, 0);
        tape.shift(Stand);
        assert_eq!(tape.index, 0);
    }
}
