# f4 - A hardware abstraction layer for STM32F4x microcontrollers

This crate is heavily inspired by `jparic`'s phenomenal work on
[svd2rust][svd2rust] and [rtfm][rtfm], as well as the many supporting libraries
he has designed. The underlying `stm32f40x` library is generated from the
manufacturer's SVD file with minimal changes using the `svd2rust` project.

The `f4` crate is a work-in-progress, and the API is subject to change. I am
currently building it out as the foundation for a project and will be
adding features and refactoring as the project demands. My hope is that the end
product will be useful to others as well.

The primary goals of the `f4` crate are:

- to make a thread-safe access to the STM32F4's peripherals
- to provide an ergonomic and readable interface to the hardware
- to provide useful debugging tools and informative error messages
- to provide an interface that is as close as possible to idiomatic Rust

[svd2rust]: https://github.com/japaric/svd2rust
[rtfm]: https://github.com/japaric/cortex-m-rtfm
