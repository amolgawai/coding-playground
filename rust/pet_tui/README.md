# Build a TUI app in rust

[source](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/)
[reference repo](https://github.com/zupzup/rust-commandline-example/blob/main/src/main.rs)

## My Notes
This was a bit heavy lifting considering my current knowledge of rust. Initially I started with coding bu towards the end, copy pasted code.
Even after copy pasting, I gained some insights into how rust works and how some crates work.
The reference code is quite a bit of spaghetti and although I factored it a bit, need more work to make it maintainable.

### Refactoring ideas
- Put the UI builder/executor code in a Ui module
- Put the pet logic in a Pets module