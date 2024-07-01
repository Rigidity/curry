# CLVM Curry Tool

Install [Rustup](https://rustup.rs).

Then, run this to install this tool:

```bash
cargo install --git https://github.com/rigidity/curry
```

Now you can curry CLVM like so:

```bash
curry '(+ 2 5)' '(1000)'
```

You can run the output with something like clvm_tools_rs:

```bash
brun '(a (q 16 2 5) (c (q . 1000) 1))' '(2000)'
```

Optionally, curry from hex:

```bash
curry -x ff10ff02ff0580 ff8203e880
```
