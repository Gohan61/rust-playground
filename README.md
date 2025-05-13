# Rust playground repository

This repository contains projects I have developed using Rust based on books and/or my own interests. Below you can find more information per project.

## The Rust Programming Language - Steve Klabnik and Carol Nichols

<details open>
<summary>Guessing game</summary>

This guessing game is based on the example given [here](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html) in this book. This project uses a newer version of the [rand](https://crates.io/crates/rand) library crate.

### Run the game

On a system with Rust installed, clone and go into the directory in a terminal:

Either run:

- cargo run (development version)
- cargo run --release (release version)

Development version compiles faster while release version runs faster. However for this small game difference is marginal.
</details>

<details>
<summary>Common collections</summary>

### Chapter 8.3 - Exercises under summary

Exercises consist mostly of working Vectors, strings and hash maps. Accessing, storing and modifying data is practiced. Visit chapter [here](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html)

### Run the binaries

Either run:

- cargo run --bin  rust-brown-book-8-3-excercise/pig_latin/employee_sort (development version)
- cargo run --release --bin rust-brown-book-8-3-excercise/pig_latin/employee_sort (release version)

</details>
