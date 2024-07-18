extern crate adapter;
extern crate adapter_extism;
extern crate extism;

use adapter::Adapter;
use adapter_extism::ExtismAdapter;

fn main() {
    let uri = "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm";

    let mut adapter = ExtismAdapter::from_url(uri).unwrap();

    let identifier = "count_vowels";
    let input = "Hello, world!";

    let output: &str = adapter.call(identifier, input).unwrap();

    assert_eq!(output, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);

    let generic_output = generic_function(&mut adapter, identifier, input).unwrap();

    assert_eq!(
        generic_output,
        r#"{"count":3,"total":6,"vowels":"aeiouAEIOU"}"#
    );
}

fn generic_function<'a, T>(
    adapter: &'a mut T,
    identifier: T::Identifier,
    input: &'a str,
) -> Result<&'a str, T::Error>
where
    T: Adapter<'a, &'a str, &'a str> + 'a,
{
    adapter.call(identifier, input)
}
