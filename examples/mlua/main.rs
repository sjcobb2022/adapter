extern crate adapter;
extern crate mlua;

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

impl<'a, Input, Output, Identifier> Adapter<'a, Input, Output, Identifier> for MLuaAdapter
where
    Input: IntoLuaMulti<'a>,
    Output: FromLuaMulti<'a>,
    Identifier: for<'b> AsChunk<'b, 'a>,
{
    type Error = mlua::Error;

    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
        self.lua.load(identifier).call::<Input, Output>(input)
    }
}

fn main() {
    let mut adapter = MLuaAdapter::new();

    let out: u32 = adapter.call("return 25", ()).unwrap();
    println!("{:?}", out);
}
