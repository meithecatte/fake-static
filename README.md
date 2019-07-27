rustc hates him! Sidestep borrow checking with this weird trick.

**Note:** This "may" cause undefined behavior. But it's fine, there's no unsafe in sight.

```rust
fn main() {
    let boom = fake_static::make_static(&vec![0; 1<<20]);
    println!("{:?}", boom);
}
```

Please don't use this in production. Or anywhere, for that matter. The only purpose of this ~~piece of shit~~ is to bring attention to the relevant issue, which hasn't seen any attention for over a year: https://github.com/rust-lang/rust/issues/25860
