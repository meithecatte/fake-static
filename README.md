> WARNING: This is a shitpost. If that is not immediately clear to you, and if it
> sounds like something you might want to use, **stop whatever you're doing immediately**
> and go read the [Rustonomicon](https://doc.rust-lang.org/nomicon/) again and again
> until you can recite it backwards while balancing on one leg.

rustc hates him! Sidestep borrow checking with this weird trick.

**Note:** This "may" cause undefined behavior. But it's fine, there's no unsafe in sight ;3

```rust
fn main() {
    let boom = fake_static::make_static(&vec![0; 1<<20]);
    println!("{:?}", boom);
}
```

Please don't use this in production. Or anywhere, for that matter. The only purpose of this ~~piece of shit~~ is to bring attention to the relevant issue, which hasn't seen any attention for over a year: https://github.com/rust-lang/rust/issues/25860
