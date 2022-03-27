//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::{assert_eq, println};

use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);


// #[wasm_bindgen_test]
// fn test_greet() {
//     greet("100");
// }

// #[cfg(test)]
// pub fn input_spaceship() -> Universe {
//     let mut universe = Universe::new();

//     universe.set_width(10);
//     universe.set_height(10);
//     universe.set_cells(&[(1,2), (2,3), (3,1), (4,2), (3, 2)]);
    
//     universe
// }

// #[cfg(test)]
// pub fn expected_spaceship() -> Universe {
//     let mut universe = Universe::new();

//     universe.set_width(10);
//     universe.set_height(10);
//     universe.set_cells(&[(2,1), (2,3), (3,2), (4,1), (4, 3)]);
    
//     universe
// }


#[wasm_bindgen_test]
pub fn test_tick() {
    // let mut  input_universe = input_spaceship();
    // let expected_universe = expected_spaceship();

    println!("test_tick()")
    // input_universe.tick();
    // assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}







