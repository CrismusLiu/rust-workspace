mod calculate;

use calculate::commons;


fn main() {
    println!("相加结果：{:?}", commons::add(1, 5));
    println!("相减结果：{:?}", commons::sub(11, 5));
}
