# Solutions to Advent of Code 2022

I challenged myself to solve this year AoC using Rust 🦀. I was in the middle
of the [The Rust Programming Language](https://doc.rust-lang.org/book/) book
when AoC started, so I was quite _rusty_ with the new language (and not in the
best sense of the word).

I'm quite proud that my code style improved a lot with each day, hope I can
keep this trend 🤞. But I still have a lot to learn.


## How to run the solutions

To run my solutions to AoC 2022 you need the Rust compiler. Since I decided not
to use any crate and stick with the standard libraries, that would be enough.
But I actually used `cargo` every day of the challenge to get used to the tool.
With it you can build, run and test each solution.
Check the
[Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
section in the aforementioned book.

For example, if you want to run the solution to day one, clone this repo,
and navigate to the `day-01` folder:

```
git clone https://www.github.com/santisoler/adventofocode-2022
cd adventofcode-2022
cd day-01
```

In there you can use `cargo` to test the code (if there are tests available):

```
cargo test
```

Or run the code to obtain solutions for both days:

```
cargo run
```

## License

Copyright © 2022 Santiago Soler

Source code available through the [MIT License](LICENSE).
