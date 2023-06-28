# References and Borrowing

* You cannot mix immut and mut (if we borrow first as immutable, we can't proceed with a mut borrow, and vice-versa (scope-dependent))
* Dangling references will error
* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

Functions also accept refs, an address from which it can access out of scope data (not transferring ownership).

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

![](https://doc.rust-lang.org/book/img/trpl04-05.svg)

De-referencing is a thing "*"
>The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

The preceeding pattern is called "borrowing". s leaves scope in calculate_length, but we don't have to pass it back in the return.

We can't modify references unless they are denoted as mutable, so the following will error.

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

## Mutable References

One big rule: you can't have multiple references to a mut reference. The proceeding will fail to compile. 

The reason? This avoids data-races, which can cause unpredictable behavior at run-time.

```
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

We can scope variables to have multiple references at different times, just not simultaneously.

```
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

```
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

solved by returning the string directly

```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```