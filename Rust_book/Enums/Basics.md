# Defining Enums

Enums come with a constructor that creates an instance
>That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.

```
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

We could define the proceeding enum with structs, but it would create multiple types.
>But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

You can also build methods on enums, as we did with structs!

```
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

## The Option Enum and Its Advantages Over Null Values

Rust has no null "feature", so I suppose that's why enums serve this purpose.

>The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error. However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

Option is an Enum defined by the standard library that encodes the scenario where a value could correlate to something, or nothing. Here is what the Option enum looks like from the std library.

```
enum Option<T> {
    None,
    Some(T),
}
```

We haven't talked about \<T> or "generic types". However, here it is saying it can hold at least one of any value type.

>For now, all you need to know is that <T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.

So the following is valid:

```
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```

This section was pretty convoluted in it's explanation, I think it struggled to convey this in a succinct manner, and we are also missing the generic type context.. I'll probably revisit these notes and add more context later.