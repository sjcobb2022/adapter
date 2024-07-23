use adapter::Adapter;

use mlua::{prelude::*, AsChunk};

pub struct MLuaAdapter(pub Lua);

impl MLuaAdapter {
    pub fn new() -> MLuaAdapter {
        MLuaAdapter(Lua::new())
    }
}

/// We have to be careful with this. AsChunk is implemented for a variety of types, however the
/// lifetimes vary. For example it is implemented statically for the String type. e.g. `AsChunk<'_, 'static> for String`.
/// This implies that the string will live for the entire lifetime of the program. This is not the case for the `&'a str` type, as it is
/// implemented for the lifetime of the string reference. Given that we are binding to the lifetime of the adapter, we must assert that
/// the identifier is valid for the lifetime of the adapter. This is why we use the `Identifier: for<'b> AsChunk<'b, 'a>` bound.
/// By bounding
impl<'a, Input, Output, Identifier> Adapter<'a, Input, Output, Identifier> for MLuaAdapter
where
    Input: IntoLuaMulti<'a>,
    Output: FromLuaMulti<'a>,
    Identifier: AsChunk<'a, 'a>, // the output of AsChunk should have at least the lifetime of 'lua
                                 // for AsChunk<'lua, 'a>. For other implementations, we do not
                                 // care about the lifetime of 'lua and only the lifetime of 'a.
{
    type Error = mlua::Error;

    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
        let identifier = self.0.load(identifier).into_function()?;
        identifier.call::<Input, Output>(input)
    }
}
