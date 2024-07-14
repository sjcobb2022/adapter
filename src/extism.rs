mod extism {

    pub struct ExtismPluginAdapter(extism::Plugin);

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
        fn call(
            &'b mut self,
            identifier: Identifier,
            input: Input,
        ) -> Result<Output, extism::Error> {
            self.0.call(identifier, input)
        }
    }
}
