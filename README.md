# Arbitrary code execution during compliation POC
This proof-of-concept demonstrates how Rust macros can be abused to interact with the machine that the compliation happens on. When the `do_not_compile_this_code` is opened in VS Code with the `rust-analyzer` plugin, the editor expands the `some_macro!()` macro. This macro reads then content of `~/.ssh/id_rsa_do_not_try_this_at_home` and deletes the file. This behavior also occurs when cargo build is run or when the application is run.

The key insight is that Rust macros are expanded before/during compilation, i.e. arbitrary code execution during compilation. This is a demostration that this is a huge vulnerability in the rust ecosystem that needs to be taken seriosly.

https://user-images.githubusercontent.com/7727602/226141592-21707f47-54d8-48a5-b825-e2bce3ccbbf0.mov

## Try it out yourself:
* Clone this repo
```bash
git clone https://github.com/eleijonmarck/do-not-run-this-code.git
```
- Create an SSH key at ~/.ssh/id_rsa_do_not_try_this_at_home with sample contents
```bash
echo "do not try this at home" > ~/.ssh/id_rsa_do_not_try_this_at_home
```
- Open `do_not_compile_this_code` in your IDE (eg: VSCode) with `rust-analyzer`

Once open, VSCode will analyze and index the code, including the expansion of macros, then you should see the contents of your `.ssh/id_rsa_do_not_try_this_at_home` will be deleted. 🤫 oops!

- You can trigger the same behavior at compile-time by running `cargo build` in the `do_not_compile_this_code` directory or by running the main application.
