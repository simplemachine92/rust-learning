# Control Flow

"if" must evaluate a bool, not if something exists.. (different from ruby and javascript) so this proceeding block will error.

```
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

"match" branching construct may be preferred to else if trees.. these expressions below are evaluated sequentially one at a time by the compiler, returning the first accurate expression. Yes, "if" is an expression.

```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Therefore we can use it on the right side of a "let" statement to assign a variable conditionally

```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

However, the if expression options must be of the same type, so the proceeding will error

```
let number = if condition { 5 } else { "six" };
```

I'm assuming that allowing different types in these expressions would create a lot of chaos..

Infinite loops are easy and can be stopped by the "break" keyword

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

## Loop disambiguation

We can denote loop names with "'{name}: loop {}" so we can break specific loops conditionally!

```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

## While loops follow the usual

```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```


## Looping over collection

```
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

>However, this approach is error prone; we could cause the program to panic if the index value or test condition is incorrect. For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

Pretty standard stuff..

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Or more concisely, and reversed

```
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```