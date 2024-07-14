use core::str;

trait Adapter<'a, Input, Output, Identifier, Error> {
    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Error>;
}

struct ExtismPluginAdapter(extism::Plugin);

impl ExtismPluginAdapter {
    fn new(plugin: extism::Plugin) -> Self {
        Self(plugin)
    }

    fn from_url(url: &str) -> Result<Self, extism::Error> {
        let url = extism::Wasm::url(url);
        let manifest = extism::Manifest::new([url]);
        let plugin = extism::Plugin::new(manifest, [], true)?;
        Ok(Self(plugin))
    }
}

impl<'a, 'b, Input, Output, Identifier> Adapter<'b, Input, Output, Identifier, extism::Error>
    for ExtismPluginAdapter
where
    Input: extism::ToBytes<'a>,
    Output: extism::FromBytes<'b>,
    Identifier: AsRef<str>,
{
    fn call(&'b mut self, identifier: Identifier, input: Input) -> Result<Output, extism::Error> {
        self.0.call(identifier, input)
    }
}

fn main() {
    let mut adapter = ExtismPluginAdapter::from_url(
        "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm",
    )
    .unwrap();

    // Convert &str to String
    let output: &str = adapter.call("count_vowels", "hello").unwrap();

    println!("{:?}", output);
}
