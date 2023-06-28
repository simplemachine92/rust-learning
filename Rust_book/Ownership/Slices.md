# The Slice Type

A slice is implicitly a reference to a sequence of elements in a collection instead of the data itself.

## String Slices

```
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```
![](https://doc.rust-lang.org/book/img/trpl04-06.svg)

Rust has some tidy range syntax

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

This is also valid
```
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

And if we want the entire string

```
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

These rules only apply to ASCII, we'll learn UTF-8 later in Chapter 8. Just make sure we are referencing single bytes.

>String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the “Storing UTF-8 Encoded Text with Strings” section of Chapter 8.

So we can return a reference to a slice and use that later:

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

So, the implicit type of a string literal; &str is an immutable reference to itself in the binary.

So in function params we should use references to string slices instead of references to strings 

```
fn first_word(s: &str) -> &str {
```

instead of 

```
fn first_word(s: &String) -> &str {
```