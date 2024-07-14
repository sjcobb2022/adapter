use adapter::Adapter;

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

impl<'a, 'b, Input, Output, Identifier> Adapter<'b, Input, Output, Identifier, extism::Error>
    for ExtismAdapter
where
    // TODO: Convert to generic associated impl type when stable.
    Input: ToBytes<'a>,
    Output: FromBytes<'b>,
    Identifier: AsRef<str>,
{
    fn call(&'b mut self, identifier: Identifier, input: Input) -> Result<Output, extism::Error> {
        self.0.call(identifier, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_output_as_extism() {
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
}
