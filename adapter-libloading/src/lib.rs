use adapter::Adapter;
use libloading::{Library, Symbol};

pub struct LibloadingAdapter {
    library: Library,
}

impl<'a, Input, Output, Identifier> Adapter<'a, Input, Output, Identifier, libloading::Error>
    for LibloadingAdapter
where
    // TODO: Convert to generic associated impl type when stable.
    Identifier: AsRef<[u8]>,
{
    fn call(
        &'a mut self,
        identifier: Identifier,
        input: Input,
    ) -> Result<Output, libloading::Error> {
        // may be a good idea to cache these.
        let symbol: Symbol<fn(Input) -> Output> =
            match unsafe { self.library.get(identifier.as_ref()) } {
                Ok(symbol) => symbol,
                Err(error) => return Err(error),
            };

        Ok(symbol(input))
    }
}

impl LibloadingAdapter {
    fn from_path(path: &str) -> Result<Self, libloading::Error> {
        match unsafe { libloading::Library::new(path) } {
            Ok(library) => Ok(Self { library }),
            Err(error) => Err(error),
        }
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
