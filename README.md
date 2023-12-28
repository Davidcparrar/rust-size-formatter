# Rust Fundamentals Week 3

This is the exercise for Week 3 of the [Rust Fundamentals Course](https://www.coursera.org/learn/rust-fundamentals)


In this lab, you will enhance a size formatter application in Rust. 
Use the [example code](https://github.com/alfredodeza/rust-structs-types-enums/blob/main/examples/14-match-enums/match-enum/src/main.rs) to get started and get an idea on how to use enum and match to handle different sizes. You are tasked with extending the application to allow a user to pass in a String representing size and unit, and then returning a debug representation of a struct that shows all the different representations in KB, MB, and GB . This is an example that takes an input and provides the output required:

```bash
$ cargo run "24 mb"
Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
The end result will be a GitHub repository containing the complete code for the enhanced file reader application.
```
## Learning Objectives:

1. Understand how to define and use enums and structs in Rust.

1. Practice error handling using match expressions and handling invalid input.

1. Gain familiarity with string parsing and formatting in Rust.

1. Learn how to implement the Debug trait to print debug information of a struct.

1. Enhance code readability by leveraging Rust's string formatting capabilities.


## Steps:

1. Create a new repository in your account for your Rust project. You can also use the [Rust template repository](Rust template repository
) to quickly generate the scaffolding for your project in your own account.

1. Use the [example code](https://github.com/alfredodeza/rust-structs-types-enums/blob/main/examples/14-match-enums/match-enum/src/main.rs) as a starting point

1. Add the ability to pass in any file size like "300 kb" or "12 mb"  to avoid hard-coding the sizes in the program. 

1. Use the [Rust docs](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html)
) or this sample code to get an idea on how to get the first argument in the console:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("Size is: {}.", args[0]);
}
```

## Hints:

1. This exercise is much more harder than the other weeks and you'll be applying more concepts and patterns. Here are some things to consider if you are getting stuck:

1. You'll need to split the input string to capture the number and the size. You can use the size (e.g. "kb") to match on how to process that number. 

1. The struct will need to have the derive debug attribute to print it out

1. Use impl to extend the struct to do the work on the struct for you

1. Use the example code to assist you with some of the match statements and transformations needed