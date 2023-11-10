mod types;

use std::collections::HashMap;
use derive_more::IsVariant;

type FreqTable = HashMap<usize, usize>;

struct Board<const C: usize> {
    inner: [CellState; C],
    width: usize,
    height: usize,
}

impl<const C: usize> Board<C> {
    fn new(width: usize, height: usize) -> Self {
        assert_eq!(
            width * height,
            C,
            "width * height != C"
        );

        Self {
            inner: [CellState::Dead; C],
            width,
            height,
        }
    }

    // used to be named `initialize_buffer`
    fn new_with_conf(width: usize, height: usize, conf: u64) -> Board<C> {
        Board {
            // could be improved but this is init code, idc
            inner: (0..(width * height))
                .map(move |i| CellState::from(conf & (1 << i)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            width,
            height,
        }
    }

    // was `neighbor_count`
    fn alive_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let width = self.width;
        let height = self.height;

        let xl = ((width as i32 + col as i32 - 1_i32) % width as i32) as usize;
        let xm = (width + col + 0) % width;
        let xr = (width + col + 1) % width;

        let yt = ((height as i32 + row as i32 - 1_i32) % height as i32) as usize;
        let ym = (height + row + 0) % height;
        let yb = (height + row + 1) % height;
        // let (xl, xm, xr, yt, ym, yb) = dbg!((xl, xm, xr, yt, ym, yb));

        (self.inner[xl + yt * width]) as u8
        + (self.inner[xm + yt * width]) as u8
        + (self.inner[xr + yt * width]) as u8
        + (self.inner[xl + ym * width]) as u8
        + (self.inner[xr + ym * width]) as u8
        + (self.inner[xl + yb * width]) as u8
        + (self.inner[xm + yb * width]) as u8
        + (self.inner[xr + yb * width]) as u8
    }
}

impl From<u64> for CellState {
    fn from(value: u64) -> Self {
        if value == 0 {
            CellState::Dead
        // } else if value == 1 {
        //     CellState::Alive
        } else {
            CellState::Alive
            // panic!("we speak ones and zeroes only")
        }
    }
}

impl From<bool> for CellState {
   fn from(value: bool) -> Self {
        match value {
            true => CellState::Alive,
            false => CellState::Dead,
        }
   } 
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, IsVariant)]
enum CellState {
    Alive = 0,
    Dead = 1,
}

fn main() {
    let frequencies: FreqTable = find_cycles::<16>(4, 4);

    println!("{:#?}", frequencies);
}

fn find_cycles<const C: usize>(width: usize, height: usize) -> FreqTable {
    let mut freq_table = HashMap::new();
    let mut board2 = Board::<C>::new(width, height);

    let mut i = 1;
    while i < (i << (width * height)) {
        let mut board1 = Board::<C>::new_with_conf(width, height, i);
        let (start, end) = find_cycle(&mut board1, &mut board2,  0, 0);

        i += 1;
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
    mut cycle_start: usize,
    mut cycle_end: usize,
) -> (usize, usize) {
    let mut turn: bool = true;
    let mut frame: usize = 0;

    let mut state_dict: HashMap<usize, usize> = HashMap::new();

    loop {
        let (from, to): (&mut Board<C>, &mut Board<C>) = match turn {
            true => (buffer1, buffer2),
            false => (buffer2, buffer1),
        };

        let s = next_gen(from, to);

        state_dict
            .entry(s as usize)
            .and_modify(|c| {
                cycle_start = *c;
                cycle_end = frame;
            })
            .or_insert(frame);

        if state_dict.get(&(s as usize)).is_some() {
            return (*state_dict.get(&(s as usize)).unwrap(), frame);
        } else {
            state_dict.insert(s as usize, frame);
        }


        turn = !turn;
        frame += frame;
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
            next.inner[index] = (n == 3 || (n == 2 && current.inner[index].is_dead())).into();

            // println!("{:?}", index);
            if next.inner[index].is_alive() {
                state = state | (1 << index);
            }
            // println!("{:?}", index);
    });

    state
}
