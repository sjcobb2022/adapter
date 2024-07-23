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
        let mut adapter = MLuaAdapter::new();

        let out: i64 = adapter.call("return 25", ()).unwrap();

        assert_eq!(out, 25);
    }

    #[test]
    fn it_can_call_basic_functions() {
        let mut adapter = MLuaAdapter::new();

        {
            let lua = &adapter.0;

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

        assert!(!out.unwrap());

        let out: Result<bool, LuaError> = adapter.call(
            r#"return check_equal({"a", "b", "c"}, {"a", "b", "c"})"#,
            (),
        );

        assert!(out.unwrap());
    }
}
