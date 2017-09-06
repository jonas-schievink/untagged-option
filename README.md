# `untagged-option`

[![Build Status](https://travis-ci.org/jonas-schievink/untagged-option.svg?branch=master)](https://travis-ci.org/jonas-schievink/untagged-option) [![Crates.io](https://img.shields.io/crates/v/untagged-option.svg)](https://crates.io/crates/untagged-option) [![](https://docs.rs/untagged-option/badge.svg)](https://docs.rs/untagged-option)

This crate provides the `UntaggedOption` type, an unsafe alternative to the existing `Option`.

In contrast to `Option`, `UntaggedOption` does not have a discriminant and thus does not know whether it contains a value or not, which makes the type very unsafe to use. It's the user's responsibility to only call `UntaggedOption`'s methods when appropriate.

`UntaggedOption` is useful in contexts where the discriminant of `Option` would consume significant amounts of memory (eg. microcontrollers). Building a safe abstraction on top of it allows safe and resource-friendly usage.
