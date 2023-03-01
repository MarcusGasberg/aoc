use std::{fs::read_to_string};

use itertools::Itertools;


struct RingBuffer<const N: usize> {
     char_window: [Option<char>; N],
     index: usize,
}

impl<const N: usize> RingBuffer<N> {
    const SIZE: usize = N;
    fn new(char_window: [Option<char>; N], index: usize) -> Self { Self { char_window, index } }

    fn push(&mut self, value: &char) -> () {
        self.char_window[self.index] = Some(*value);
        self.index += 1;
        if self.index >= self.char_window.len() {
            self.index = 0;
        }
    }

    fn any(&self, value: char) -> bool {
        self.char_window.iter().any(|v| match v { Some(x) => *x == value, _ => false })
    }

    fn all_unique(&self) -> bool {
        self.char_window.iter().all_unique()
    }

    fn size(&self) -> usize {
        Self::SIZE
    }

    fn print(&self) {
        println!("{:?}", self.char_window);
    }
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self::new([None; N], 0)
    }
}



pub fn main() {
    let Ok(input) = read_to_string("src/days/day6_input.txt") else {return };
    let mut ring_buffer: RingBuffer<14> = RingBuffer::default();

    let x = input.chars().enumerate().fold(None, |mut acc, (i, val)| {
        ring_buffer.print();
        ring_buffer.push(&val);
        if acc == None && ring_buffer.all_unique() && i > ring_buffer.size() {
            acc = Some(i + 1);
        }

        return acc;
    });

    println!("{}", x.unwrap());
}

