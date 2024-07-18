use adapter::Adapter;

use mlua::{prelude::*, AsChunk};

struct MLuaAdapter {
    lua: Lua,
}

impl MLuaAdapter {
    fn new() -> Self {
        MLuaAdapter { lua: Lua::new() }
    }
}

impl<'b, Input, Output> Adapter<'b, Input, Output> for MLuaAdapter
where
    // TODO: Convert to generic associated impl type when stable.
    // Identifier: for<'c, 'd> AsChunk<'c, 'd>,
    Input: IntoLuaMulti<'b>,
    Output: FromLuaMulti<'b>,
{
    type Error = mlua::Error;
    type Identifier = &'b str
        where Self::Identifier: for<'d> AsChunk<'d, 'b>;

    fn call(
        &'b mut self,
        identifier: Self::Identifier,
        input: Input,
    ) -> Result<Output, Self::Error> {
        self.lua.load(identifier).call::<Input, Output>(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_return_data_from_lua() {
        let mut adapter = MLuaAdapter::new();

        let out: i64 = adapter.call("return 25", ()).unwrap();

        assert_eq!(out, 25);
    }

    #[test]
    fn it_can_call_to_stdio() {
        let mut adapter = MLuaAdapter::new();
        {
            let lua = &adapter.lua;

            let globals = lua.globals();

            let check_equal = lua
                .create_function(|_, (list1, list2): (Vec<String>, Vec<String>)| {
                    // This function just checks whether two string lists are equal, and in an inefficient way.
                    // Lua callbacks return `mlua::Result`, an Ok value is a normal return, and an Err return
                    // turns into a Lua 'error'. Again, any type that is convertible to Lua may be returned.
                    Ok(list1 == list2)
                })
                .unwrap();

            globals.set("check_equal", check_equal).unwrap();
        }

        let out: Result<bool, LuaError> = adapter.call(
            r#"return check_equal({"a", "b", "c"}, {"d", "e", "f"})"#,
            (),
        );

        assert_eq!(out.unwrap(), false);

        let out: Result<bool, LuaError> = adapter.call(
            r#"return check_equal({"a", "b", "c"}, {"a", "b", "c"})"#,
            (),
        );

        assert_eq!(out.unwrap(), true);
    }
}
