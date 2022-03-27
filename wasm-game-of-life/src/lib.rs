mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate fixedbitset;
// use fixedbitset::FixedBitSet;

extern crate web_sys;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    // cells: FixedBitSet,
}

// 打印log
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $($t)* ).into());
    }
}

#[wasm_bindgen]
impl Universe {

    pub fn new() -> Universe {
        // utils::set_panic_hook();

        let width = 64;
        let height = 64;
        
        let cells = (0..width * height).map(|_| {
            if js_sys::Math::random() < 0.5 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        // FixedBitSet版本
        // let size = (width * height) as usize;
        // let mut cells = FixedBitSet::with_capacity(size);
        
        // for i in 0..size {
        //     // cells.set(i, i % 2 == 0 || i % 7 == 0);
        //     cells.set(i, js_sys::Math::random() < 0.5);
        // }

        Universe { width, height, cells }
    }
    
    // 通过row和column，获取指定cell的下标
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * &self.width + column) as usize
    }


    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!("cell [{},{}] is initially {:?} and has {} live neighbors",
                //     row, col,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            
                next[idx] = next_cell;

                // FixedBitSet版本
                // let next_cell = match(cell, live_neighbors) {
                //     (true, x) if x < 2 => false,
                //     (true, 2) | (true, 3) => true,
                //     (true, x) if x > 3 => false,
                //     (false, 3) => true,
                //     (otherwise, _) => otherwise,
                // };
                // // log!("    cell [{},{}] becomes {:?} ", row, col, next_cell);
                // next.set(idx, next_cell);
            }
        }
        
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // 点击
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    // FixedBitSet版本
    // pub fn cells(&self) -> *const u32 {
    //     self.cells.as_slice().as_ptr()
    // }

    // test使用
    // pub fn set_width(&mut self, width: u32) {
    //     self.width = width;
    //     self.cells = (0..width * self.height).map(|_| 0).collect();
    // }

    // pub fn set_height(&mut self, height: u32) {
    //     self.height = height;
    //     self.cells = (0..height * self.width).map(|_| 0).collect();
    // }

}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                // FixedBitSet版本            
                // let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }    

        Ok(())
    }
}

// FixedBitSet版本
impl Universe {


    // 获取存活邻居个数
    fn live_neighbor_count_old(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_raw in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_raw == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_raw) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // 行：上
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        // 行：下
        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        // 列：左
        let west = if col == 0 {
            self.width - 1
        } else {
            col - 1
        };

        // 列：右
        let east = if col == self.width - 1 {
            0
        } else {
            col + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let nc = self.get_index(north, col);
        count += self.cells[nc] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;



        let rw = self.get_index(row, west);
        count += self.cells[rw] as u8;

        // let rc = self.get_index(row, col);
        // count += self.cells[rc] as u8;

        let re = self.get_index(row, east);
        count += self.cells[re] as u8;



        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let sc = self.get_index(south, col);
        count += self.cells[sc] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    
//     pub fn get_cells(&self) -> &FixedBitSet {
//         &self.cells
//     }

//     pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
//         for (row, col) in cells.iter().cloned()  {
//             let idx = self.get_index(row, col);
//             self.cells.set(idx, true);
//         }
//     }
}


impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}

// ========================= time profiling =========================

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer {name}
    }
}


impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}


