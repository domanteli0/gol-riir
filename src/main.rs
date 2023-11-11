mod types;

use std::collections::HashMap;
use std::fmt::Debug;
// use std::error::Error;
use std::fmt::Formatter;
use crate::types::CellState;

type FreqTable = HashMap<usize, usize>;

#[derive(Clone, Copy)]
struct Board<const C: usize> {
    inner: [CellState; C],
    width: usize,
    height: usize,
}

impl<const C: usize> Debug for Board<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut f = f.debug_list();
        for h in 0..self.height {
            f.entry(
                &&(self.inner[(h*self.width)..((h+1)*self.width)])
            );
        }

        f.finish()
    }
}

impl<const C: usize> Board<C> {
    const fn new(width: usize, height: usize) -> Self {
        if width * height != C { panic!("width * height != C") }

        Self {
            inner: [CellState::Dead; C],
            width,
            height,
        }
    }

    // // used to be named `initialize_buffer`
    fn replace_with_conf(&mut self, conf: u64) {
        for (i, cell) in self.inner.iter_mut().enumerate() {
            *cell = CellState::from(if (conf & (1 << (i as u64))) > 0 { 1 } else { 0 })
        }
    }
    
    // was `neighbor_count`
    fn alive_neighbor_count(&self, row: usize, col: usize) -> i32 {
        let width = self.width;
        let height = self.height;

        let xl = ((width as i32 + col as i32 - 1_i32) % width as i32) as usize;
        let xm = (width + col + 0) % width;
        let xr = (width + col + 1) % width;

        let yt = ((height as i32 + row as i32 - 1_i32) % height as i32) as usize;
        let ym = (height + row + 0) % height;
        let yb = (height + row + 1) % height;
        // let (xl, xm, xr, yt, ym, yb) = dbg!((xl, xm, xr, yt, ym, yb));

        (self.inner[xl + yt * width]) as   i32
        + (self.inner[xm + yt * width]) as i32
        + (self.inner[xr + yt * width]) as i32
        + (self.inner[xl + ym * width]) as i32
        + (self.inner[xr + ym * width]) as i32
        + (self.inner[xl + yb * width]) as i32
        + (self.inner[xm + yb * width]) as i32
        + (self.inner[xr + yb * width]) as i32
    }
}

fn main() {
    let frequencies: FreqTable = find_cycles::<16>(4, 4);

    println!("{:#?}", frequencies);
}

fn find_cycles<const C: usize>(width: usize, height: usize) -> FreqTable {
    let mut freq_table = HashMap::new();
    let mut board1 = Board::<C>::new(width, height);
    let mut board2 = Board::<C>::new(width, height);

    let len = 1 << C;
    for i in 0..len {
        // println!("b1: {:?}\nb2: {:?}", board1, board2);
        board1.replace_with_conf(i);
        let (start, end) = find_cycle(&mut board1, &mut board2);

        let period = end - start;
        freq_table
            .entry(period)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    freq_table
}

fn find_cycle<const C: usize>(
    buffer1: &mut Board<C>,
    buffer2: &mut Board<C>,
) -> (usize, usize) {
    let mut turn: bool = true;
    let mut frame: usize = 0;

    let mut state_dict: HashMap<u64, u64> = HashMap::new();

    loop {
        let (from, to): (&mut Board<C>, &mut Board<C>) = match turn {
            true => (buffer1, buffer2),
            false => (buffer2, buffer1),
        };

        let s = next_gen(from, to);

        if let Some(&something) = state_dict.get(&s) {
            return (something as usize, frame);
        } else {
            state_dict.insert(s, frame as u64);
        }

        turn = !turn;
        frame += 1;
    }
}

fn next_gen<const C: usize>(
    current: &mut Board<C>,
    next: &mut Board<C>,
) -> u64 {
    let mut state: u64 = 0;
    let width: usize = current.width;
    let height: usize = current.height;

    (0..height)
        .flat_map(|i| ((0..width).map(move |j| (i, j))))
        .for_each(|(i, j)| {
            // println!("{:?}, {:?}", i, j);
            let n: usize = current.alive_neighbor_count(i % height, j % width) as usize;
            let index: usize = j + i * width;
            // println!("{:?}", index);
            // println!("{:?}", next.inner[index]);
            next.inner[index] = (n == 3 || (n == 2 && current.inner[index].is_dead())).into();

            // println!("{:?}", index);
            if next.inner[index].is_alive() {
                state = state | (1 << index);
            }
            // println!("{:?}", index);
        });

    state
}
