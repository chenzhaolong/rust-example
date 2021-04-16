fn main() {
    test1()
}

// 元组, 结构体, 元组 + 结构体
fn test1() {
    struct A2 {
        b1: u32,
        b2: bool
    }

    struct A3 (i64, u8, f32);

    let a1 = (12_i32, false, "str"); // 元组
    let a2 = A2 {b1: 23_u32, b2: true}; // 结构体
    let a3 = A3 (125_i64, 5_u8, 23.45_f32); // 元组 + 结构体

    println!("元组: {} {} {}", a1.0, a1.1, a1.2);
    println!("结构体: {} {}", a2.b1, a2.b2);
    println!("元组 + 结构体, {} {} {}", a3.0, a3.1, a3.2);
}