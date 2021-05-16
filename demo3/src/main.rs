fn main() {
    let mut func : fn((i32, f32)) -> i32 = demo1;
    func = demo2;
    let _a = func((23, 32.0));
    println!("{}", _a);
    let b = demo3(3);
    println!("{}", b);
    println!("end");
}

fn demo1 (a: (i32, f32)) -> i32 {
    return 32;
}

fn demo2 (a: (i32, f32)) -> i32 {
   let b = a.1 as i32;
   return a.0 + b;
}

fn demo3 (x: i32) -> i32 {
    if x > 20 {
        demo4();
    } else {
        return demo5(x);
    }
}

fn demo4 () -> ! {
    panic!("it is error")
}

fn demo5(a: i32) -> i32 {
    if a == 1 {
        return 1;
    }
    if a == 2 {
        return 2;
    }
    return demo5(a - 1) + demo5(a - 2);
}