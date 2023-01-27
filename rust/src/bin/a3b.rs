fn main() {
    let n = 5;

    if n > 5 {
        println!("{:?}>5", n);
    } else if n < 5 {
        println!("{:?}<5", n);
    } else {
        println!("{:?}=5", n);
    }
}
