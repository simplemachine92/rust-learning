A vector stores each value next to each other in memory

```
    let v: Vec<i32> = Vec::new();
```

Rust can infer the type of your vector based on values

```
    let v = vec![1, 2, 3];
```

To add elements to a vector ```.push()```

You can access a value with it's index or with ```get()``` method. ```get()``` will return an Option type, while accessing with an index will be a reference.

```
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

basically .get() won't panic if the desired index to grab is non-existent, and instead returns ```None```.

immutable vector example:

```
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

mutable vector example:

```
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

```

## Using an Enum to store multiple types

Since vectors require all values to be of the same type, we can just use the Enum type to store values of multiple types. Rust has to know this in order to allocate the heap, and lets the compiler know what operations can be performed.

```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

Vectors are dropped once they leave scope like anything else

```
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

```