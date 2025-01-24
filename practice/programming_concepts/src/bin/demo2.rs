// 生成第 n 个斐波那契数
fn main() {
    let n = 3;
    let s = fibonacci(n);
    println!("第 {n} 个斐波那契数是: {s}");
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut prev = 0;
    let mut current = 1;
    for _ in 2..n {
        let next = prev + current;
        prev = current;
        current = next;
    }
    // 返回
    current
}
