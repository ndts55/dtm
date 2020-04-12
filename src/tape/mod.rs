#[cfg(test)]
mod tests;

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
        self.read_at(self.index)
    }

    fn read_at(&self, position: isize) -> Option<T> {
        if position < 0 {
            let index = (-(position + 1)) as usize;
            self.negative.read_at(index)
        } else {
            self.positive.read_at(position as usize)
        }
    }

    pub fn write(&mut self, new_value: Option<T>) {
        self.write_at(new_value, self.index);
    }

    fn write_at(&mut self, new_value: Option<T>, position: isize) {
        if position < 0 {
            let index = (-(position + 1)) as usize;
            self.negative.write_at(new_value, index);
        } else {
            self.positive.write_at(new_value, position as usize);
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.index -= 1,
            Direction::Right => self.index += 1,
            _ => (),
        }
    }
}

trait TapeVector<T> {
    fn read_at(&self, position: usize) -> T;
    fn write_at(&mut self, new_value: T, position: usize);
}

impl<T> TapeVector<Option<T>> for Vec<Option<T>>
where
    T: Copy,
{
    fn read_at(&self, position: usize) -> Option<T> {
        self.get(position).and_then(|&opt| opt)
    }

    fn write_at(&mut self, new_value: Option<T>, position: usize) {
        if self.len() > position {
            self[position] = new_value;
        } else {
            self.resize_with(position, || None);
            self.push(new_value);
        }
    }
}

impl<T> From<Vec<T>> for Tape<T> {
    fn from(positive: Vec<T>) -> Tape<T> {
        let positive = positive.into_iter().map(Some).collect();
        Tape {
            positive,
            negative: Vec::new(),
            index: 0,
        }
    }
}
