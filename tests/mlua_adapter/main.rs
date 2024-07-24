extern crate adapter;
extern crate mlua;

#[path = "./mod.rs"]
mod mlua_adapter;

use adapter::Adapter;
use mlua::prelude::*;
use mlua_adapter::MLuaAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_return_data_from_lua() {
        let lua = Lua::new();

        lua.globals()
            .set(
                "return_number",
                lua.create_function(|_, ()| Ok(42)).unwrap(),
            )
            .unwrap();

        let mut adapter = MLuaAdapter::from_lua(lua);

        let identifier = "return_number";
        let input = ();
        let output: i32 = adapter.call(identifier, input).unwrap();
        assert_eq!(output, 42);
    }

    #[test]
    fn it_can_call_basic_inbuilt_lua_functions() {
        let lua = Lua::new();
        let mut adapter = MLuaAdapter::from_lua(lua);
        let identifier = "type";
        let input = "test";
        let output: String = adapter.call(identifier, input).unwrap();
        assert_eq!(output, "string");
        // assert_eq!(output, 13);
    }

    #[test]
    fn it_can_mutate_data_within_function() {
        let lua = Lua::new();

        let mut number = 0;

        lua.globals()
            .set(
                "return_mutable_number",
                // We have to move our data into the scope of the function, but this is fine in
                // most cases.
                lua.create_function_mut(move |_, ()| {
                    number += 1;
                    Ok(number)
                })
                .unwrap(),
            )
            .unwrap();

        let mut adapter = MLuaAdapter::from_lua(lua);

        let identifier = "return_mutable_number";
        let input = ();
        let output: i32 = adapter.call(identifier, input).unwrap();
        assert_eq!(output, 1);

        let output: i32 = adapter.call(identifier, input).unwrap();
        assert_eq!(output, 2);
    }
}
