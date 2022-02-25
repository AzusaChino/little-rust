# little-rust

starts on 2021.06.01

the path of becoming rusty.

## Main Content

1. Learn from microsoft [path](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)
2. Following the book `<The Rust Programming Language>`
3. Other practical example codes

## Objective

Build something with rust.

## Notes

1. To fix `edition 2021` problem, run `rustup update`.
2. use cargo global `config` to enable proxy.

## Package Definition

1. Package: A cargo feature that lets you build, test and share crates
2. Crates: A tree of modules that produces a library or executable
3. Modules and use: Let you control the organization, scope, and privacy of paths
4. Paths: A way of naming an item, such as struct, function, or module

## Config CN mirror

install with mirror `curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh`

```config
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

## mdbook

I think mdbook is good project to learn rust usage, so I decided to `Ctrl+C` and `Ctrl+V` all the codes to this repo.

## About `?`

Rust handles error uses `Result`

```rust
fn fn1(i: i32) -> Result<i32, Error> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        Err(/*Something*/)
    }
}

fn do_the_thing(i: i32) {
    let i = match fn1(i) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    println!("{}", i);
}

// 1. unpacks the Result if OK
// 2. returns the error if not, calling Into::into on the error
//    value to potentially convert it to another type.
fn do_the_thing_better(i:i32) {
    let i = fn1(i)?;
    println!("{}", i);
}
```
