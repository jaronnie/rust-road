fn main() {
    let v = vec![1, 2, 3];
    
    // out of index
    // RUST_BACKTRACE=full cargo run to get more info
    v[99];
}
