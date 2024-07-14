/// The Adapter trait is used to define the interface for an adapter.
/// An adapter is a component that is used to convert the input from a client
/// into a format that is understood by the service.
pub trait Adapter<'a, Input, Output, Identifier, Error> {
    // TODO: Use associated types?
    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
}
