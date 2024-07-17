use adapter::Adapter;
use libloading::{Library, Symbol};

pub struct LibloadingAdapter {
    library: Library,
}

impl LibloadingAdapter {
    fn from_path(path: &str) -> Result<Self, libloading::Error> {
        match unsafe { libloading::Library::new(path) } {
            Ok(library) => Ok(Self { library }),
            Err(error) => Err(error),
        }
    }
}

impl<'a, Input, Output> Adapter<'a, Input, Output> for LibloadingAdapter {
    type Error = libloading::Error;
    type Identifier = &'a [u8];

    fn call(&'a mut self, identifier: Self::Identifier, input: Input) -> Result<Output, Self::Error>
    where
        Self::Identifier: AsRef<[u8]>,
    {
        // may be a good idea to cache these.
        let symbol: Symbol<fn(Input) -> Output> =
            match unsafe { self.library.get(identifier.as_ref()) } {
                Ok(symbol) => symbol,
                Err(error) => return Err(error),
            };

        Ok(symbol(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_libloading_works() {
        // TODO: impl testing

        // let mut adapter = LibloadingAdapter::from_path("some_path").unwrap();
        //
        // let res:  = adapter
        //     .call("some_identifier", "some_input".to_string())
        //     .unwrap();
    }
}
