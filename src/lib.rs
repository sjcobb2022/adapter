pub mod adapt {
    pub trait Adapter<'a, Input, Output, Identifier, Error> {
        fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Error>;
    }
}
