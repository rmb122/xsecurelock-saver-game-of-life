use rand::Rng;
use std::cell::RefCell;
use std::ops::Not;
use std::vec;

#[derive(Copy, Clone, PartialEq)]
pub enum LifeStatus {
    Dead,
    Alive,
}

impl Not for LifeStatus {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LifeStatus::Alive => LifeStatus::Dead,
            LifeStatus::Dead => LifeStatus::Alive,
        }
    }
}

pub struct Life {
    board: RefCell<Vec<Vec<LifeStatus>>>,
    rng: rand::rngs::ThreadRng,
}

pub struct LifeStatusDiff {
    pub x: u16,
    pub y: u16,
    pub status: LifeStatus,
}

impl Life {
    pub fn new(size_x: u16, size_y: u16) -> Self {
        Life {
            board: RefCell::new(vec![
                vec![LifeStatus::Dead; size_x as usize];
                size_y as usize
            ]),
            rng: rand::thread_rng(),
        }
    }

    pub fn initialize(&mut self, alive_probability: f64) -> Vec<LifeStatusDiff> {
        let mut diffs: Vec<LifeStatusDiff> = Vec::new();

        for (y, row) in self.board.borrow_mut().iter_mut().enumerate() {
            for x in 0..row.len() {
                if self.rng.gen_range(0f64..=1f64) <= alive_probability {
                    row[x] = LifeStatus::Alive;

                    diffs.push(LifeStatusDiff {
                        x: x as u16,
                        y: y as u16,
                        status: LifeStatus::Alive,
                    })
                } else {
                    row[x] = LifeStatus::Dead;
                }
            }
        }
        return diffs;
    }

    fn get_cell_status_next_round(board: &Vec<Vec<LifeStatus>>, x: u16, y: u16) -> LifeStatus {
        let mut count = 0u32;
        let size_y = board.len();
        let size_x = board[0].len();

        for y_offset in -1i32..=1i32 {
            for x_offset in -1i32..=1i32 {
                if y_offset == 0 && x_offset == 0 {
                    continue;
                } else if board[(y_offset + (y as i32)).rem_euclid(size_y as i32) as usize]
                    [(x_offset + (x as i32)).rem_euclid(size_x as i32) as usize]
                    == LifeStatus::Alive
                {
                    count += 1
                }
            }
        }

        match count {
            3 => LifeStatus::Alive,
            2 => board[y as usize][x as usize],
            _ => LifeStatus::Dead,
        }
    }

    pub fn add_mutation(&mut self, mutation_probability: f64) -> Vec<LifeStatusDiff> {
        let mut diffs: Vec<LifeStatusDiff> = Vec::new();

        for (y, row) in self.board.borrow_mut().iter_mut().enumerate() {
            for x in 0..row.len() {
                if self.rng.gen_range(0f64..=1f64) <= mutation_probability {
                    row[x] = !row[x];
                    diffs.push(LifeStatusDiff {
                        x: x as u16,
                        y: y as u16,
                        status: row[x],
                    })
                }
            }
        }
        return diffs;
    }

    pub fn next_round(&self) -> Vec<LifeStatusDiff> {
        let mut diffs: Vec<LifeStatusDiff> = Vec::new();

        for (y, row) in self.board.borrow().iter().enumerate() {
            for x in 0..row.len() {
                let cell_next_status =
                    Life::get_cell_status_next_round(&self.board.borrow(), x as u16, y as u16);

                if cell_next_status != row[x] {
                    diffs.push(LifeStatusDiff {
                        x: x as u16,
                        y: y as u16,
                        status: cell_next_status,
                    })
                }
            }
        }

        let mut board_mut = self.board.borrow_mut();
        for diff in diffs.iter() {
            board_mut[diff.y as usize][diff.x as usize] = diff.status;
        }

        return diffs;
    }
}
