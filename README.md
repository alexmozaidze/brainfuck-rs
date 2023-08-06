# brainfuck-rs

A fast Brainfuck interpreter written in Rust.

## Specification

This implementation of Brainfuck complies with the spec that can be found [here](https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md).

## Features

- *fast*: doesn't use recursion internally
- *safe*: no `unsafe` keyword
- *configurable tape size*: 30000 by default
- *interactive/piped input support*

## Bugs

- `EOF` goes undetected in Brainfuck programs (see #1)
