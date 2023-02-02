fn main() {
    let mut n = 1;

    loop {
        let nn = n * n;

        if nn >= 200 { break; }

        println!("{} ", nn);

        n += 1;
    }
}
