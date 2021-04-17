enum Number {
    Int(i32),
    FL(f32),
}
enum IP{
    V6(i32),
    V4(i32, i32),
    V2,
    V1 {
        a: i32,
        b: f32
    }
}

fn main() {
    test1();
    let num = Number::FL(23.012);
    test2(&num);
    let ip = IP::V4(23, 56);
    test3(ip);
    let ip1 = IP::V2;
    test3(ip1);
    let ip2 = IP::V1{a: 12, b: 12.32};
    test3(ip2);
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

// 枚举类
fn test2(num: &Number) {
    match num {
        &Number::Int(val) => println!("{}", val),
        &Number::FL(val) => println!("{}", val),
    }
    let a: (i32, f32) = (23, 1.0);
    println!("Number的内存占用空间{}", std::mem::size_of::<Number>());
    println!("Int的内存占用空间{}", std::mem::size_of::<i32>());
    println!("FL的内存占用空间{}", std::mem::size_of::<f32>());
    println!("{}", a.1);
}

fn test3(ip: IP) {
    match ip {
        IP::V1{a, b} => println!("a1 {} a2 {}", a, b),
        IP::V4(val, val1) => println!("V4: {}", val1),
        IP::V6(val) => println!("V6: {}", val),
        IP::V2 => println!("V2:{}", 123),
       
    }
}