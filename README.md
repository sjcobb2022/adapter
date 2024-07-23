# Adapter 

> [!WARNING]  
> This library is in no way production ready.

Adapter is a library meant for providing a standard interface for which one can communicate with a variety of interfaces or plugins. It was created from a desire to be able to configure programs with a variety of different languages.

To see the thought process behind this, see [RAMBLINGS.md](./RAMBLINGS.md).

The key concept of this library is the Adapter trait. It hinges on the idea that communicating with a foreign interface should be as simple as a function that has an input and an output. We use an identifier to determine the "endpoint" that we want to access.


```rust
pub trait Adapter<'a, Input, Output, Identifier> {
    type Error;

    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error>;
}
```

An adapter is generic ovpub trait Adapter<'a, Input, Output, Identifier, Error> {
    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Error>;
}er the input, output, identifier and error that is uses. It takes in an identifier and an input, and returns a result with the output or an error. By being generic, we can require stricter traits be applied to our inputs and outputs.
