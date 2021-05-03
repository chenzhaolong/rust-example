fn main() {
    let res = demo1(16_i32, 17_i32, 12_i32);
    println!("{}", res);
    
    demo2();

    demo3(122);

    demo4(1_i32);

    demo5();
}

// 连续比较
fn demo1 (a: i32, b: i32, _c: i32) -> bool {
    // return (a == b == c);
    return a == b;
}


fn demo2 () {
    let a: u8 = 0b_10101010;
    let b: u8 = 0b_11110000;
    println!("{}", !a);
    println!("{}", a & b);
    println!("{}", a | b);
    println!("{:08b}", a ^ b);

    let mut _c = 1;
    _c += 1;
    println!("{}", _c);
}

fn demo3 (a: i32) -> i32 {
    let a1 = if a > 12 {true} else {false};
    println!("{}", a1);
    if a > 32 {
        println!("asdf")
    }
    return 12;
} 

fn demo4 (mut a: i32) {
    loop {
        a += 1;
        if a > 12 {
            break;
        } else {
            println!("{}", a);
        }
    }
}

fn demo5 () {
    let a = &[1,2,3,4,5];
    for i in a {
        println!("i is {}", i);
    }
}