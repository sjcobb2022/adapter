use adapter::Adapter;

use mlua::prelude::*;

pub struct MLuaAdapter(pub Lua);

impl MLuaAdapter {
    pub fn new() -> MLuaAdapter {
        MLuaAdapter(Lua::new())
    }

    pub fn from_lua(lua: Lua) -> MLuaAdapter {
        MLuaAdapter(lua)
    }
}

/// We have to be careful with this. AsChunk is implemented for a variety of types, however the
/// lifetimes vary. For example it is implemented statically for the String type. e.g. `AsChunk<'_, 'static> for String`.
/// This implies that the string will live for the entire lifetime of the program. This is not the case for the `&'a str` type, as it is
/// implemented for the lifetime of the string reference. Given that we are binding to the lifetime of the adapter, we must assert that
/// the identifier is valid for the lifetime of the adapter. This is why we use the `Identifier: for<'b> AsChunk<'b, 'a>` bound.
/// By bounding
impl<'lua, Input, Output, Identifier> Adapter<'lua, Input, Output, Identifier> for MLuaAdapter
where
    Input: IntoLuaMulti<'lua>,
    Output: FromLuaMulti<'lua>,
    Identifier: IntoLua<'lua>, // the output of AsChunk should have at least the lifetime of 'lua
                               // for AsChunk<'lua, 'a>. For other implementations, we do not
                               // care about the lifetime of 'lua and only the lifetime of 'a.
{
    type Error = mlua::Error;

    fn call(&'lua mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
        let lua = &self.0;
        let globals = lua.globals();
        let func: mlua::Function = globals.get(identifier)?;
        func.call::<Input, Output>(input)
    }
}
