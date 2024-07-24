extern crate adapter;

mod extism_adapter;
mod mlua_adapter;

use adapter::Adapter;
use extism_adapter::ExtismAdapter;
use mlua_adapter::MLuaAdapter;

use extism::{convert::Json, FromBytes, ToBytes};
use mlua::{FromLua, FromLuaMulti, IntoLuaMulti};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct VowelData {
    count: i32,
    total: i32,
    vowels: String,
}

impl<'lua> FromLua<'lua> for VowelData {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => Ok(ud.borrow::<Self>().unwrap().to_owned()),
            _ => unreachable!(),
        }
    }
}

impl mlua::UserData for VowelData {}

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
    K: FromLuaMulti<'b>,
    Json<K>: FromBytes<'b>,
{
    adapter.call(identifier, input)
}

fn less_generic_function<'b, T>(
    adapter: &'b mut T,
    identifier: &'b str, // The same is true here, &'b str is a common type that satisfies all
    // bounds.
    input: String,
) -> Result<String, T::Error>
where
    T: Adapter<'b, String, String, &'b str>,
{
    adapter.call(identifier, input.to_string())
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn count_vowels(s: &str) -> usize {
    match s.chars().next() {
        None => 0,
        Some(x) => {
            let count_rest = count_vowels(&s[1..]);
            if is_vowel(x) {
                count_rest + 1
            } else {
                count_rest
            }
        }
    }
}

fn setup_lua() -> mlua::Lua {
    let mut lua = mlua::Lua::new();

    let mut total = 0;

    lua.globals()
        .set(
            "count_vowels",
            lua.create_function_mut(move |_, input: String| {
                let count = count_vowels(&input);
                total += count;
                Ok(VowelData {
                    count: count as i32,
                    total: total as i32,
                    vowels: "aeiouAEIOU".to_string(),
                })
            })
            .unwrap(),
        )
        .unwrap();

    lua
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn it_works_with_extism_and_less_generic() {
        let uri = "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm";
        let mut adapter = ExtismAdapter::from_url(uri).unwrap();
        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();
        let out: String = less_generic_function(&mut adapter, identifier, input).unwrap();

        assert_eq!(out, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);
    }

    #[test]
    fn it_works_with_mlua_and_generic() {
        let mut lua = setup_lua();

        let mut adapter = MLuaAdapter::from_lua(lua);

        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();

        let out: VowelData = generic_function(&mut adapter, identifier, input).unwrap();

        assert_eq!(
            out,
            VowelData {
                count: 3,
                total: 3,
                vowels: "aeiouAEIOU".to_string(),
            }
        );

        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();

        let out: VowelData = generic_function(&mut adapter, identifier, input).unwrap();

        assert_eq!(
            out,
            VowelData {
                count: 3,
                total: 6,
                vowels: "aeiouAEIOU".to_string(),
            }
        );
    }

    #[test]
    fn it_works_with_mlua_and_less_generic() {
        let mut adapter = MLuaAdapter::new();

        let identifier = "count_vowels";
        let input = "Hello, world!".to_string();

        // unimplemented!();
        // let out: String = less_generic_function(&mut adapter, identifier, input).unwrap();
    }
}
