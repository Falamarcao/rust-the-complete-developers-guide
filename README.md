# Rust: The Complete Developer's Guide

| Status      | Start Date | End Date |
| ----------- | ---------- | -------- |
| In Progress | 2026-02-24 | N/A      |

> <https://www.udemy.com/course/rust-the-complete-developers-guide/>

## Running

```sh
cd projects/<project_name>

cargo run

# quiet mode (Do not print cargo log messages)
cargo run - q
```

## Notes

- `Strings` are represented by double quotes `"Hello World"`, while single quotes `'` are used for representing char (character literal);
- `#[derive[Debug]]` tell the compiler to add a list of functions from `Debug` trait to our program;
- `Vec` vectors (e.g. `vec!["Ace", "Two", "Three"]`) are lists that can grow, `[]` Arrays (e.g. `["Ace", "Two", "Three"]`) have fixed lenght;
- Arrays have a slight performance improvement over Vectors;
- Variables  are called bindings in rust; `mut` keywords makes a `binding` mutable;
- `imp` keyword is for `Inherent Implementations` (add a function to a struc), define `methods` and `associated functions` (In other programming lanaguges a.k.a. `class method`);
- `Implicit return` - Rust will return the last executed expression if not end with a `;` (semicolon), this also works inside code blocks like if statements.
- Rust does not have imports, dependencies are resolved at root, and don't require imports, however we have `use` keyword to pull specific things into the scope of the file, like `use rand::rng` and using direcly like `let rng = rng();` or `use rand::{rng, random, rngs::OsRng};` or `use rand;` and use like `let rng = rand::rng();`
