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
