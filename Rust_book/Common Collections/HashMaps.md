## Creating a Hash Map

Hash maps are included in the collections portion of the std library ```use std::collections::HashMap;``` hash map data is stored on the heap

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

We can get the value of a key using the get method. If there is no value, this will return None, just like in vectors get method.

```
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

We can iterate over each key/value pair. This will print each in an arbitrary order.

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

Moving ownable variables into a hashmap means it takes ownership when they are inserted.

```
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```

So what happens with references? The values the references point to must be valid as long as the hashmap is valid, or the compiler will error.

## Overwriting

Inserts are sequential so whatever appears latest in the code is valid.

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

```

## Adding a key and value if a key doesnt exist

We have an entry method on hashmaps that returns an enum called ```Entry```

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```