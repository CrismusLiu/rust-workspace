use rand::{thread_rng, Rng};
use std::{error::Error, ops::Add};

use customer_err_lib as ErrLib;

// mod lib;

// use main_action::testCal;


fn main() {
    // testLocalLibErr();
    // testOutLibErr();

    testOutLibFromErr();

    // testCal();

}



// lib库在当前项目中，需要先引入lib的mod
// fn testLocalLibErr() {
//     match lib::get_super_error() {
//         Err(e) => {
//             println!("Error: {}", e);
//             println!("Caused by: {}", e.source().unwrap());
//         }
//         _ => println!("No error"),
//     }
// }

// 使用外部err的库
// fn testOutLibErr() {
//     match ErrLib::get_super_error() {
//         Err(e) => {
//             println!("Error: {}", e);
//             println!("Caused by: {}", e.source().unwrap());
//         }
//         _ => println!("No error"),
//     }
// }


fn testOutLibFromErr() {
    let ss = ErrLib::open_and_parse_file("d:/data/num.txt1");
    println!("{:#?}", ss);
}