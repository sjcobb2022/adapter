/// The Adapter trait is used to define the interface for an adapter.
/// An adapter is a component that is used to convert the input from a client
/// into a format that is understood by the service.
pub trait Adapter<'a, Input, Output> {
    type Error;
    type Identifier;
    // type Input;
    // TODO: Use associated types?
    // The input and output types should be generic over the function call,
    // as they may differ.
    //
    // The identifier and error can be static.
    fn call(
        &'a mut self,
        identifier: Self::Identifier,
        input: Input,
    ) -> Result<Output, Self::Error>;
}
