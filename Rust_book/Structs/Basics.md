# Defining and Instantiating

First we define a struct by issuing a name and type to it's fields.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Then we can instantiate it like this using key: value pairs. We don't have to specify them in order.

```
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

We access struct values via dot notation (like in solidity), we can modify values if the instance of the struct is mutable.
>Note: We can not specify specific fields to be mutable or immutable, only the entire instance.

```
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

We can implicitly return struct instances from functions.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

We can shorthand the field defining syntax like so:

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

And even more "struct update shorthand". This updates the email value but imports all other fields from user1 instance.

```
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Tuple Structs are used for creating multiple duplicate types with different names while not being overly verbose.

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Unit-Like Structs Without Any Fields

I'm a bit confused on this right now, but apparently this is elaborated upon in Chapter 10. I suppose it acts as placeholder for test data.

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```