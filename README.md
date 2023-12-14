# brainfuck-rs

A simple Brainfuck interpreter written in Rust.

> Although, Brainfuck is a fairly useless language, I wanted to test myself in making an easy-to-use interpreter.

## Features

### Stack-based

It doesn't use recursion internally, it uses a stack, which entirely avoids stack overflow errors.

### Safe

It doesn't use `unsafe` keyword. This pretty much means that it is unlikely to cause undefined behaviour.

### Flexible

You can use brainfuck-rs both as a library and a standalone executable.

#### Library

You can specify the input and output buffers using [`BufReader`] and [`BufWriter`] respectively, but you can use anything that implements [`Read`] and [`Write`] traits.

#### Standalone executable

Use `brainfuck-rs -h` to view all the options that can be used.

### Scriptable

First-class support for scripting, allowing you to pipe input into a Brainfuck program.

Here's an example:
```sh
echo 'Hello, World!' | brainfuck-rs rot13.b

# or

brainfuck-rs rot13.b <<< 'Hello, World!'
```

You can find plenty of Brainfuck programs to experiment with inside [examples/brainfuck-programs](examples/brainfuck-programs).

## Performance

This implementation does not introduce any optimizations, which means that it simply executes instructions character by character, but it's fast enough for most use cases (if you find one). For instance, [mandelbrot.b](examples/brainfuck-programs/mandelbrot.b) finishes in 1 minute 48 seconds on Pentium dual-core (`Pentium E5200 (2) @ 2.500GHz`).

I didn't want to overcomplicate the implementation, so I tried to keep things as simple as possible.

## Specification Compliance

This implementation of Brainfuck tries to comply with the spec that can be found [here](https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md).

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`BufReader`]: https://doc.rust-lang.org/stable/std/io/struct.BufReader.html
[`BufWriter`]: https://doc.rust-lang.org/stable/std/io/struct.BufWriter.html
[`usize::MAX`]: https://doc.rust-lang.org/stable/std/primitive.usize.html
