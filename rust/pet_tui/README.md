# Build a TUI app in rust

[reference article](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/)
[reference repo](https://github.com/zupzup/rust-commandline-example/blob/main/src/main.rs)

## My Notes

This was a bit heavy lifting considering my current knowledge of rust. Initially I started with coding bu towards the end, copy pasted code.
Even after copy pasting, I gained some insights into how rust works and how some crates work.
The reference code is quite a bit of spaghetti and although I factored it a bit, need more work to make it maintainable.

### Improvements over the reference

- [x] Handle missing db file
- [x] Use latest rand module
- [x] Use a real pet name generator (petnames) over gibberish produced by rand
- [x] Refactor main into smaller functions
- [x] Create db directory if doesn't exist
- [x] Handle deleting all entries
- [x] Handle crash on empty pet list

#### Refactoring ideas

- Put the UI builder/executor code in a Ui module
- Put the pet logic in a Pets module
