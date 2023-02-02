fn main() {
    let mut n = 1;
    while true {
        let n2 = n * n;

        if n2 >= 200 { break; }

        println!("{}", n2);

        n += 1;
    }
}

