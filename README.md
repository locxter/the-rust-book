# The Rust book

## Overview

> Please keep in mind that this is a **work in progress** and **not my primary focus** anymore.

This repository includes my personal implementations of most code examples given in the [Rust book](https://doc.rust-lang.org/book/).

## Dependencies

I generally try to minimize dependencies, but I'm a one man crew and can therefore only support Debian-based Linux distributions as I'm running one myself. Anyway, you need to have the following packages installed for everything to work properly:

- rustc and cargo for compiling and managing the Rust projects. Install them with `sudo apt install build-essential gdb rust-all rust-src`.

## How to use this

The code examples are organized into their respective `chapter/subchapter/project` folders and should all be working without further changes. To execute an example, simply navigate to the desired project and run `cargo run` from the terminal. Happy learning!
