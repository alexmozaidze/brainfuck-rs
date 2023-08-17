# brainfuck-rs

A fast Brainfuck interpreter written in Rust.

## Specification

This implementation of Brainfuck complies with the spec that can be found [here](https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md).

## Features

### no recursion

It doesn't use recursion internally, it uses a stack, which entirely avoids stack overflow errors.

### safe

It doesn't use `unsafe` keyword. This pretty much means that it is unlikely to cause undefined behaviour.

### configurability

You can specify the desired length of the tape. By default, the tape length is 30000, but you may set it to anything ranging from 1 all the way to `usize::MAX`.

If you're using this crate as a library, you can specify the input and output buffers using [`BufReader`](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html) and [`BufWriter`](https://doc.rust-lang.org/stable/std/io/struct.BufWriter.html).

### flexibility

brainfuck-rs supports interactive

```sh
brainfuck-rs
```

and piped input

```sh
echo "+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]" | brainfuck-rs
```

along with file input

```sh
brainfuck-rs some-brainfuck-program.b
```
