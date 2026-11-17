# Overview

I'm trying to understand Rust better, I wanted to do so by focusing on mutability, references and scope, which is unique in how it's handled compared to C++ through its ownership and borrowing system to enforce memory safety. Working on input and output interactions with a file gave me the opportunity to test several of those aspects in Rust.

I wrote a simple file manager program in Rust that lets me read from a file, add lines and apply individual line edits to a file using a vector of stored file strings in Rust.

The purpose of the software was to expand my horizons as a software engineer by using a language I'm unfamiliar with. What drew me to Rust was that it's similar to C++ in many ways but designed to enforce memory safety.

[Software Demo Video](https://youtu.be/Ma2uvK1IlFY)

# Development Environment

Visual Studio Code, rustc (Rust's compiler), and Cargo (dependency manager).

I used the built in standard fs (file system) and io (input/output) Rust libraries.

# Useful Websites

- [YouTube (Learn Rust Programming - Complete Course)](https://www.youtube.com/watch?v=BpPEoZW5IiY)
- [doc.rust-lang.org](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

# Future Work

- Let the user pick a file in the folder to edit.
- Folder navigation
- When editing the line include the line to edit as a placeholder instead of starting with a blank line.