extern crate adapter;
extern crate libloading;

#[path = "./mod.rs"]
mod libloading_adapter;

use adapter::Adapter;
use libloading::Error;

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
