fn main() {
    knowArray();
}

fn knowArray () {
    // 创建数组
    let mut a: [i32; 3] = [1,2,3];
    println!("first: {}", a[1]);
    // 数组比较
    let b: [i32; 3] = [1,2, 3];
    println!("second: {}", b == a);
    // 获取数组的值
    println!("third {}", b[0] + a[1]);
    let c: [i32; 3] = [1,2,5];
    a = c;
    for i in &a {
        println!("{}", i)
    }
    let s: &mut [i32; 3] = &mut a;
    Slice(s);
    println!("s:{}", s[2]);
    println!("a:{}", a[2]);
}

fn Slice(a: &mut [i32]) {
    a[2] = 5;
    println!("slice{}", a[2])
}

fn Fat(a: &[i32]) {
    println!("Fat{}", a[1]);
}