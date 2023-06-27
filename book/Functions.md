## Functions

Rust has expressions and statements

Statements do not return a value, expressions do.

You end a statement with ;

New Scope Blocks are a thing:

```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Functions are expressions

Parameters and return values of functions can be typed

Implicit returns are a thing but returns can happen early with the "return" keyword

```
fn five() -> i32 {
    5
}
```
