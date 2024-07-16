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

    let ours: &str = adapter.call(identifier, input).unwrap();

    println!("{}", ours);
}
