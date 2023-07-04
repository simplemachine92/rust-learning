# Match vs if

If you recall, ifs must evaluate to a boolean. In a match it can be any type.

Matches.. well.. match values to expressions. A value mapped to an expression is called an "arm".

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## "If let" as an alternative

Instead of the more verbose match and arm pattern.

>you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

```
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

this also supports an else conditional

```
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```