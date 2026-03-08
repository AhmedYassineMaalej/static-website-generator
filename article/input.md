---
title: Making an SSG with Rust
---

For now this article will contain a general walkthrough of how I made this tool.

## Runtime

tokio and stuff for multithreading


## File Watching

[notify.rs](https://github.com/notify-rs/notify) crate


## Bulding the HTML

html_builder crate

## Hot Reloading

websockets with tungstenite

## Code highlighting
tried treesitter (pain) then arborium

```rust
struct User {
    id: usize,
    name: String
}

fn main() {
    let x = 1;
    prinln!("hello world");
}
```


```rust
struct User {
    id: usize,
    name: String
}

fn main() {
    prinln!("hello world");
}
```


```rust
struct User {
    id: usize,
    name: String
}

fn main() {
    prinln!("hello world");
}
```


```rust
struct User {
    id: usize,
    name: String
}

fn main() {
    prinln!("hello world");
}
```

```rust
struct User {
    id: usize,
    name: String
}

fn main() {
    prinln!("hello world");
}
```



