fn main() {
    let boom = fake_static::make_static(&vec![0; 1<<20]);
    println!("{:?}", boom);
}
