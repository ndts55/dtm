use dtm::*;

fn main() {
    let d = |state: i32, _: Option<char>| (state + 1, Some('a'), Direction::Right);

    println!("{:?}", DTM::new(0, 10, 11, d).unwrap());
    println!("{:?}", DTM::new(0, 10, 11, delta).unwrap());
    println!(
        "{:?}",
        DTM::new(0, 10, 11, |state: i32, _: Option<char>| (
            state + 1,
            Some('a'),
            Direction::Right
        ))
        .unwrap()
    );
}

fn delta(state: i32, _: Option<char>) -> (i32, Option<char>, Direction) {
    (state + 1, Some('a'), Direction::Right)
}
