fn main() {
    let mut func : fn((i32, f32)) -> i32 = demo1;
    func = demo2;
    let _a = func((23, 32.0));
    println!("{}", _a);
}

fn demo1 (a: (i32, f32)) -> i32 {
    return 32;
}

fn demo2 (a: (i32, f32)) -> i32 {
   let b = a.1 as i32;
   return a.0 + b;
}