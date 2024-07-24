extern crate adapter;
extern crate extism;

#[path = "./mod.rs"]
mod extism_adapter;

use adapter::Adapter;
use extism::{Manifest, Plugin, Wasm};
use extism_adapter::ExtismAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_the_same_output_as_extism() {
        let uri = "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm";

        let url = Wasm::url(uri);
        let manifest = Manifest::new([url]);
        let mut plugin = Plugin::new(manifest, [], true).unwrap();

        let mut adapter = ExtismAdapter::from_url(uri).unwrap();

        let identifier = "count_vowels";
        let input = "Hello, world!";

        let ours: &str = adapter.call(identifier, input).unwrap();

        let theirs = plugin.call::<&str, &str>(identifier, input).unwrap();

        assert_eq!(ours, theirs);
    }

    #[test]
    fn it_can_do_multiple_calls() {
        let uri = "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm";

        let mut adapter = ExtismAdapter::from_url(uri).unwrap();

        let identifier = "count_vowels";
        let input = "Hello, world!";

        let first: &str = adapter.call(identifier, input).unwrap();
        let first = first.to_owned();
        let second: &str = adapter.call(identifier, input).unwrap();
        let second = second.to_owned();

        assert_ne!(first, second);

        assert_eq!(first, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);
        assert_eq!(second, r#"{"count":3,"total":6,"vowels":"aeiouAEIOU"}"#);
    }
}
