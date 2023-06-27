# Guessing Game

First we grab a standard library to take cmd line input.

[Standard Module](https://doc.rust-lang.org/std/prelude/index.html)

```
use std::io;
```

And use our boring printlns that we already know about.
```
    println!("Guess the number!");

    println!("Please input your guess.");
```

We initialize a mutable string to hold our input from the user
```
    let mut guess = String::new();
```

>The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.

read_line appends to a string, & indicates a [Reference](https://doc.rust-lang.org/stable/core/primitive.reference.html) to our string. We have to reference it with a mutable quality since this is appending to the string (mutating it).
```
    io::stdin()
        .read_line(&mut guess)
```

read_line returns a Result, an enum type that expresses different states, in our example here (ok, err). Result has an expect method you can call. The compiler will warn you if you forget to handle your errors.

```
.expect("Failed to read line");
```

## Generating a secret number

add rand to our dependencies in cargo.toml
```
[dependencies]
rand = "0.8.5"
```