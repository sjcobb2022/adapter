# Adapter 

> [!WARNING]  
> This library is in no way production ready.

Adapter is a library meant for providing a standard interface for which one can communicate with a variety of interfaces or plugins. It was created from a desire to be able to configure programs with a variety of different languages.

To see the thought process behind this, see [RAMBLINGS.md](./RAMBLINGS.md).

The key concept of this library is the Adapter trait. It hinges on the idea that communicating with a foreign interface should be as simple as a function that has an input and an output (similar to a tower service). We use an identifier to determine the "endpoint" that we want to access.


```rust
pub trait Adapter<'a, Input, Output, Identifier> {
    type Error;

    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error>;
}
```

An adapter is generic over the input, output, identifier that is uses. It must take the input, and an identifier, and return a result with the output and the error.

Any restrictions for types must be applied in the implementation of the type (that is also probably generic over the input, output and identifier).

When using multiple adapters, one must constrain the input, output and identifier to types that meet the requirements of all the adapter's in use. Common types for these may be `String` and `&str`.
