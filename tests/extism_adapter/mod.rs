extern crate adapter;
extern crate extism;

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

impl<'b, Input, Output, Identifier> Adapter<'b, Input, Output, Identifier> for ExtismAdapter
where
    Input: ToBytes<'b>,
    Output: FromBytes<'b>,
    Identifier: AsRef<str>,
{
    type Error = extism::Error;

    fn call(&'b mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
        self.0.call::<Input, Output>(identifier, input)
    }
}
