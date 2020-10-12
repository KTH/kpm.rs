# kpm

This is attempt to create a (switchable) standalone application to
provide the KTH Personal Menu.
It is also a small experiment / demonsteration for doing stuff in Rust
at KTH GVS IT.

## Development and running

For local development, the standrad Rust `cargo` tool can be used for
compiling, testing and running.
The continous integration builds a Docker image that can be used in
the staging and production environments (which does not need to know
that the application is written in rust).
