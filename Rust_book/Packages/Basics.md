## Overview

The module system entails

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

"Crates" are the smallest amount of code that the compiler considers at a time. There are library crates and binary crates.

* Library crates don't compile to an executable or have a main function. It's intended to be used across multiple projects, a lib is colloquially a "crate" in the Rust community.
* A package is a bundle of crates that provides some intended functionality. Cargo is an example of a package.
* A package can contain multiple binary crates and a single library crate.
* For example, when you create a package with "Cargo new", you instantiate a crate binary in src/main.rs

## Modules and submodules

Modules and submodules are imported with the following type of syntax

```
use crate::garden::vegetables::Asparagus;
```

That is how you would use the module garden, the submodule vegetables, and the type \<Asparagus>

You can instantiate a package with a library using

```
cargo new restaurant --lib
```

Code within a module is private by default.

Structs and Enums as well as their fields can be made public, and therefore accessible by other modules, we must also specify the fields privacy.

```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

This works because we provide the necessary associated function that instantiates a Breakfast correctly. If not, this would not compile as we cannot set seasonal_fruit outside of the module. This allows for necessary some granular modularity.

## Bringing paths into scope

Shortcutting:

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

The proceeding won't compile because the use statement is outside the scope of the module, we could fix it by moving the use inside of the in-line declared module "customer"

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

>Note: bringing the module into scope itself and using "hosting::add_to_waitlist();" is the most idiomatic way, if we bring the specific function into scope it would be difficult to grok where "add_to_waitlist()" is declared.

If we are bringing in two identical types, we distinguish them by indirect reference as well, otherwise the proceeding code won't compile.

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

You can make this more declarative importing "as".

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

You can re-export imports with ```pub use```

Multiple items with the same prefix:

```
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

Also using the self keyword we can bring std::io and std::io::Write into scope shorthanded:

```
use std::io::{self, Write};
```

and lastly globs bring everything into scope. ```use std::collections::*;```