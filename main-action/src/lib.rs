use std::{error::Error, ops::Add};
use std::fmt;
use rand::{thread_rng, Rng};

pub mod calculate;
use calculate::cal;

#[derive(Debug)]
pub struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
pub struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for SuperErrorSideKick {}

pub fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError { side: SuperErrorSideKick })
}

// 分模块：将一个大文件拆分成一个模块
pub fn testCal() {
    let res = cal::add(thread_rng().gen_range(1..100), 2);
    println!("加法运算：{:?}", res);

    let res = cal::sub(thread_rng().gen_range(1..100), 3);
    println!("减法运算：{:?}", res);
}