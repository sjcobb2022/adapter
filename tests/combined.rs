extern crate adapter;

mod extism_adapter;
mod mlua_adapter;

use adapter::Adapter;
use extism::FromBytes;
use extism::ToBytes;
use extism_adapter::ExtismAdapter;
use mlua::FromLuaMulti;
use mlua::IntoLuaMulti;
use mlua_adapter::MLuaAdapter;

#[cfg(test)]
mod tests {

    use mlua::AsChunk;

    use super::*;

    fn generic_function<'b, T, I, J, K>(
        adapter: &'b mut T,
        identifier: I,
        input: J,
    ) -> Result<K, T::Error>
    where
        T: Adapter<'b, J, K, I>,
        I: AsRef<str>, // Careful, our identifier bound must satisfy all of the adapters trait
        // bounds. This means we need a common type that satisfies all adapter inputs, outputs and identifiers.
        J: IntoLuaMulti<'b> + ToBytes<'b>,
        K: FromLuaMulti<'b> + FromBytes<'b>,
    {
        adapter.call(identifier, input)
    }

    fn less_generic_function<'b, T>(
        adapter: &'b mut T,
        identifier: String, // The same is true here, &'b str is a common type that satisfies all
        // bounds.
        input: String,
    ) -> Result<String, T::Error>
    where
        T: Adapter<'b, String, String, String>,
    {
        adapter.call(identifier, input.to_string())
    }

    #[test]
    fn it_works_with_extism_and_generic() {
        let uri = "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm";
        let mut adapter = ExtismAdapter::from_url(uri).unwrap();
        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();
        let out: String = generic_function(&mut adapter, identifier, input).unwrap();

        assert_eq!(out, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);
    }

    #[test]
    fn it_works_with_mlua_and_generic() {
        let mut adapter = MLuaAdapter::new();

        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();

        unimplemented!();

        let out: String = generic_function(&mut adapter, identifier, input).unwrap();
    }

    #[test]
    fn it_works_with_extism_and_less_generic() {
        let uri = "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm";
        let mut adapter = ExtismAdapter::from_url(uri).unwrap();
        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();
        let out: String =
            less_generic_function(&mut adapter, identifier.to_string(), input).unwrap();

        assert_eq!(out, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);
    }

    #[test]
    fn it_works_with_mlua_and_less_generic() {
        let mut adapter = MLuaAdapter::new();
        let identifier = "count_vowels".to_string();
        let input = "Hello, world!".to_string();
        unimplemented!();
        let out: String = less_generic_function(&mut adapter, identifier, input).unwrap();
    }
}
