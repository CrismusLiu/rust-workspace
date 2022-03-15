mod calculate;
pub use crate::calculate::cal;

use rand::{thread_rng, Rng};
use std::error::Error;

use customer_err_lib as ErrLib;

// mod lib;


fn main() {
    // testLocalLibErr();
    // testOutLibErr();

    testOutLibFromErr();

}

// 分模块：将一个大文件拆分成一个模块
// fn testCal() {
//     let res = cal::add(thread_rng().gen_range(1..100), 2);
//     println!("加法运算：{:?}", res);

//     let res = cal::sub(thread_rng().gen_range(1..100), 3);
//     println!("减法运算：{:?}", res);
// }

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