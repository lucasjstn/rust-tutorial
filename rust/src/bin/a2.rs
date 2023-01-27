fn main() {
    let result = sum(34, 3478);

    display_result(result)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}
