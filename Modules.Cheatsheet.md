# Modules Cheat Sheet

## Start from the crate root

When compiling a crate, the compiler first looks in the crate root file (usually *src/lib.rs* for a library crate or *src/main.rs* for a binary crate) for code to compile.

## Declaring modules

In the crate root file, you can declare new modules; say you declare a "garden" module with `mod garden;`. The compiler will look for the module's code in these places:

- Inline, within curly brackets that replace the semicolon following `mod garden`;
- In the file *src/garden.rs*;
- In the file *src/garden/mod.rs*;

## Declaring submodules

In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in *src/garden.rs*. Then the compiler will look for the submodule's code within the directory named for the parent module in these places:

- Inline, directly following the `mod vegetables`, within curly braces instead of the semicolon;
- In the file *src/garden/vegetables.rs*;
- In the file *src/garden/vegetables/mod.rs*;

## Path to code in modules

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

## Private vs. public

Code within a module is private from its parent modules by default. To make it public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.

## The `use` keyword

Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

## Example

Here, we create a binary crate named `backyard` that illustrates these rules. The crate's directory, also named `backyard`, contains these files and directories:

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │    └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case in *src/main.rs*, and it contains:

Filename: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in *src/garden.rs*, which is:

Filename: src/garden.rs

```rust
pub mod vegetables;
```

Here, `pub mod vegetables;` means the code in *src/garden/vegatables.rs* is included too. That code is:

Filename: src/garden/vegetables.rs

```rust
#[derive(Debug)]
pub struct Asparagus {}
```