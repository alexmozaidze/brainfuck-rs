# brainfuck-rs

A fast Brainfuck interpreter written in Rust.

## Specification

This implementation of Brainfuck tries to complies with the spec that can be found [here](https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md).

## Features

### 1. no recursion

It doesn't use recursion internally, it uses a stack, which entirely avoids stack overflow errors.

### 2. safe

It doesn't use `unsafe` keyword. This pretty much means that it is unlikely to cause undefined behaviour.

### 3. configurability

By default, the tape length is 30000, but you may set it to anything ranging from 1 all the way to [`usize::MAX`](https://doc.rust-lang.org/stable/std/primitive.usize.html).

If you're using this crate as a library, you can specify the input and output buffers using [`BufReader`](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html) and [`BufWriter`](https://doc.rust-lang.org/stable/std/io/struct.BufWriter.html).

### 4. flexibility

brainfuck-rs not only supports interactive input, but piped input as well, which introduces scripting:

```sh
echo "Hello, World!" | brainfuck-rs rot13.b # Prints "Uryyb, Jbeyq!"
```

You can find plenty of Brainfuck programs to experiment with inside [examples/brainfuck-programs](examples/brainfuck-programs).

## Who Asked?

Although, Brainfuck is a fairly useless language, I wanted to test myself in making an interpreter and handling such a program gracefully and elegantly, which I did.

So, in short: I did.
