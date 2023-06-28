# Method Syntax

>Unlike functions, methods are defined within the context of a struct (or an enum or a trait object)

In the proceeding example we use an impl (implementation) block for our Rectangle struct, wherein we define a method "area". Using "self." we can reference the struct fields and denote operations that can be performed on them. Then, we create an instance of Rectangle in our main function, and we are able to use our method. I love how convenient it is to define this.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

I'd say the only confusing portion of this code is when we use a self reference, which has a lengthy but convoluted explanation.. I'll just provide the last statement from that explanation as it's more succinct.

```
fn area(&self) -> u32 {
```

>Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

Also

>We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.

>We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

Methods can accept multiple parameters

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

## Associated Functions

>All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.

Notice that this associated function does not have a &self parameter (thus it is distinguished from a method)

```
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

To call associated functions we use the :: syntax with the Struct name, like so.

```
let sq = Rectangle::square(3);
```
