use adapter::{Adapter, AdapterInput, AdapterOutput};

use extism::{FromBytes, Manifest, Plugin, ToBytes, Wasm};

pub struct ExtismAdapter(Plugin);

impl ExtismAdapter {
    pub fn new(plugin: Plugin) -> Self {
        Self(plugin)
    }

    pub fn from_url(url: &str) -> Result<Self, extism::Error> {
        let url = Wasm::url(url);
        let manifest = Manifest::new([url]);
        let plugin = Plugin::new(manifest, [], true)?;
        Ok(Self(plugin))
    }
}

impl<'b, Input, Output> Adapter<'b, Input, Output> for ExtismAdapter
where
    Input: ToBytes<'b>,
    Output: FromBytes<'b>,
{
    type Error = extism::Error;
    type Identifier = &'b str;

    fn call(
        &'b mut self,
        identifier: Self::Identifier,
        input: Input,
    ) -> Result<Output, Self::Error> {
        self.0.call::<Input, Output>(identifier, input)
    }
}

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
