fn main() {
    let truth = true;
    let falsity = false;

    println!("{} {}", !truth, !falsity);
    println!("{} {} {} {}", falsity && falsity, falsity && truth, truth && falsity, truth && truth);

    println!("{} {} {} {}", falsity || falsity, falsity || truth, truth || falsity, truth || truth);
}