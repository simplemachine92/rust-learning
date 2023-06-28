# The Stack and The Heap

Stack: LIFO, push, pop, fixed/known size at compile time.  
Heap: Dynamic allocation, pointers (can be stored on the stack), slower access than stack, unknown at compile time*?

How this works for in scope of a function with pointers on the stack:
>When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

The proceeding example can be mutated and lives on the heap [Method Syntax](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#:~:text=more%20in%20the-,%E2%80%9CMethod%20Syntax%E2%80%9D,-section%20of%20Chapter)
```
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
```

>With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

The important thing to note here is that we aren't performing manual garbage collection, once a heap stored data is out of scope, it's allocation is automatically pruned.

```
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
```

This seems simple but can lead to complications in more complex patterns

```
    let s1 = String::from("hello");
    let s2 = s1;
```

![](https://doc.rust-lang.org/book/img/trpl04-02.svg)

So, how is pruning handled when either variable goes out of scope here?  
To avoid "double freeing" errors and memory security vulnerabilities, Rust considers "s1" as no longer valid once s2 is declared. 

So we don't really have a "shallow copy" like in other langs, but instead a move from s1 -> s2 done by the compiler. When attempting to access s1 after s2 declared, the compiler will throw
>move occurs because `s1` has type `String`, which does not implement the `Copy` trait

So, even if you're code is ugly and non-idiomatic, we can still assume a decent run-time.

## What if we want to clone?

The proceeding code copies using the common method clone, which actually creates a copy in the heap
```
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

![](https://doc.rust-lang.org/book/img/trpl04-03.svg)

## Always an Exception

This is valid, guess why?
```
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

Because types with a known size are stored entirely on the stack.

All scalar values work this way:

* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating-point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Why call it Ownership?

Because of scoping, examples:

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

Returning values also changes ownership

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```