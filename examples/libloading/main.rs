extern crate adapter;
extern crate libloading;

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

impl<'a, Input, Output, Identifier> Adapter<'a, Input, Output, Identifier> for LibloadingAdapter
where
    Identifier: AsRef<[u8]>,
{
    type Error = libloading::Error;

    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
        // may be a good idea to cache these.
        let symbol: Symbol<fn(Input) -> Output> =
            match unsafe { self.library.get(identifier.as_ref()) } {
                Ok(symbol) => symbol,
                Err(error) => return Err(error),
            };

        Ok(symbol(input))
    }
}

fn main() {
    println!("Unimplemeted");
}
