# Ruscord - Net

This crate is designed for HTTP async interaction.
Fetch HTTP URL (with `urlops`) asynchronously.
Result of asynchronous response is `future` instance.
And this crate also supports waiting for `future` in synchronous function—it will block that function.
