# rust-exercises

> A collection of hopefully useful code exercises in Rust language

## Motivation

Often, when approaching a new (for us) programming language, we tend to make at least one of the following mistakes:
* we read a lot of documentation without trying to apply concepts in code step by step;
* we try to apply to code from the beginning without reading enough documentation;
* we rely on blog articles rather than the official documentation;
* we rely too much on code snippets found on the web or some textbook instead of trying to implement something useful given the amount of knowledge we have got so far.

This time, being Rust not the most trivial of programming languages, I choose to structure my own exercises as methodically as it sounds reasonable to do for the basic coding you'll se.
I'll try to develop each example with order and purpose in mind, with a fair amount of comments and avoiding complexity.
> I embrace the **KISALAICBKS** (Keep It Simple As Long As It Can Be Kept Simple) logic.
> If one of the examples is going to become more complex than needed, it will become a separate repository.

## Installation/Usage

There is nothing to be installed, the code is mainly intented to be read --- it would be not that useful otherwise.
The repository includes a separate folder for each binary example or library of functions.
Say you are in the repo's main directory and you see a subfolder named `some_directory`.
Expect a binary example when you find a `some_directory/src/main.rs`, and a library of function with unit tests when you find a `root_directory/src/lib.rs` --- it's that simple!
Run `cargo run` in the former case (try the toy program) and `cargo test` in the latter case (check if all the functions in the library pass the unit tests).

## Progress

### Libraries

- [ ] `basic-algorithms`
  > A collection of the most common algorithms everyone should when preparing for a technical interview
  - [ ] binary trees
    - [ ] basic implementation of the data structure
	- [ ] breadth first search
	- [ ] depth first search
	- [ ] inversion
	- [ ] ...
  - [ ] graphs
    - [ ] basic implementation of the data structure
	- [ ] breadth first search
	- [ ] depth first search
	- [ ] Dijkstra
	- [ ] reversal
	- [ ] ...
  - [ ] hash tables
    - [ ] basic implementation of the data structure
	- [ ] ...
  - [ ] searching algorithms
    - [x] `binary_search`
	- [x] `linear_search`
	- [x] ...
  - [ ] sorting algorithms
    - [x] `bubble_sort`
    - [x] `merge_sort`
    - [x] `quick_sort`
  - [ ] ...

- [ ] ... The best is yet to come!

### Binary examples

- [ ] `bill-the-splitter`
> a simple GUI application relying on GTK bindings: to impress your housemates when you need to split bills!

- [x] `factorial`
> computes the factorial of a given number; it accepts the input as a command line argument, but if run without arguments will prompt the user for input --- it can deal with very large inputs, far larger than the ones I could manage on the same machine with C++ or Julia...

- [x] `tree`
> an implementation of the classic `tree` command with a reduced set of options

- [ ] ... The best is yet to come!

## License

[MIT](https://choosealicense.com/licenses/mit/)
