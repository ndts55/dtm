use super::Direction;

#[derive(Debug)]
pub struct Tape<T> {
    positive: Vec<Option<T>>,
    negative: Vec<Option<T>>,
    index: isize,
}

impl<T> Tape<T>
where
    T: PartialEq + Copy,
{
    pub fn read(&self) -> Option<T> {
        (if self.index < 0 {
            self.negative.get(((self.index - 1) * -1) as usize)
        } else {
            self.positive.get(self.index as usize)
        })
        .and_then(|&opt| opt)
    }

    pub fn write_and_move(&mut self, new_value: Option<T>, direction: Direction) {
        if self.read() != new_value {
            self.write(new_value);
        }
        self.motion(direction);
    }

    fn write(&mut self, new_value: Option<T>) {
        if self.index < 0 {
            self.write_negative(new_value);
        } else {
            self.write_positive(new_value);
        }
    }

    fn write_positive(&mut self, new_value: Option<T>) {
        for _ in self.positive.len() as isize..self.index - 1 {
            self.positive.push(None);
        }

        self.positive.push(new_value);
    }

    fn write_negative(&mut self, new_value: Option<T>) {
        let index = self.index * -1;
        for _ in self.negative.len() as isize..index - 1 {
            self.negative.push(None);
        }

        self.negative.push(new_value);
    }

    fn motion(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.index -= 1,
            Direction::Right => self.index += 1,
            _ => (),
        }
    }
}

impl<T> From<Vec<T>> for Tape<T> {
    fn from(positive: Vec<T>) -> Tape<T> {
        let positive = positive.into_iter().map(|t| Some(t)).collect();
        Tape {
            positive,
            negative: Vec::new(),
            index: 0,
        }
    }
}
