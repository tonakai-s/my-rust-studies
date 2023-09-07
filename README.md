# my-rust-studies
A simple repository where I put my own tests while learning Rust reading the [Rust Book](https://doc.rust-lang.org/book/title-page.html).

## Cargo

```cargo new PROJECT_NAME ``` Create a new project.

```cargo build``` => Build the current project, after this the executable file will be available on /target folder.

```cargo run``` => Compile the current project and run it, on the same command.

```cargo check``` => Check the code to make sure it compiles, but not run it.

## Diferences between Statements and Expressions.
Statements => Are instructions that perform some actions and do not return a value.
Expressions => Evaluate to a resultant value.

## Tests
```rust
pub fn add_two(value: i32) -> i32 {
    if(value < 0){
        panic!("Number need be positive");
    }
    value + 2
}

#[cfg(test)]
mod test {
    use super::*

    #[test]
    fn my_test_function(){
        let result = add_two(4);
        assert_eq!(result, 4);
        assert_ne!(result, 5);
    }

    #[test]
    #[should_panic(expected="need be positive")]
    fn my_panic_test(){
        add_two(-1);
    }

    #[test]
    #[ignore]
    fn this_test_will_be_ignored(){
        add_two(-2);
    }
}
```

```#[cfg(test)]``` -> Tells to compiler to run tests code only on run ```cargo test```, not on ```cargo build```.

### Integration tests
All integration tests go on Tests directory, on the top of the project.
```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

To create a code to be used on more than on test, an not run it as test itself, we use the pattern with mod.rs.
```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

## Tests useful flags
### For all moments
All are initiated by ```$ cargo test```
```-- --test-threads=1``` -> By default, tests are runned on parallel, this force to Rust compiler use only 1 thread.

```-- --show-output``` -> By default, ```println!``` macros only inside the functions being testeds, only appear on tests who fail, this force to appear on ALL tests.

```{HERE_IS_THE_TEST_NAME}``` -> This code run only the test passed as argument.

```{HERE_IS_PART_OF_THE_TEST_NAME}``` -> Run all tests who match with this substring in his name.

```--test``` -> Run only integration tests, we can specify a test name, or a file name to run all tests on it.

### When using the derive ```#[ignore]```

```-- --ignored``` -> Run only ignored tests.

```-- --inclued-ignored``` -> Run all tests, including ignored tests.



Common questions.
Why print! execute after asking for an input when declared before?