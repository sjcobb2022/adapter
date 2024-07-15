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

impl<'a, 'b, Input, Output, Identifier> Adapter<'b, Input, Output, Identifier, mlua::Error>
    for MLuaAdapter
where
    // TODO: Convert to generic associated impl type when stable.
    Identifier: for<'c, 'd> AsChunk<'c, 'd>,
    Input: IntoLuaMulti<'b>,
    Output: FromLuaMulti<'b>,
{
    fn call(&'b mut self, identifier: Identifier, input: Input) -> Result<Output, mlua::Error> {
        self.lua.load(identifier).call::<Input, Output>(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut adapter = MLuaAdapter::new();
        //
        // let out: () = adapter.call("print", "Hello World!").unwrap();
    }
}
