/// The Adapter trait is used to define the interface for an adapter.
/// An adapter is a component that is used to convert the input from a client
/// into a format that is understood by the service.
///
/// By making the Input, Output and Identifier generic, the end user must restrict
/// the types that can be used with the adapter.
///
/// The implementation's of an adapter can further restrict generic types.
/// For example, we can restrict a lua adapter to only work with types that can be
/// converted to and from lua, and have a valid identifier. See [mlua adapter example](../examples/mlua/main.rs).
///
///
/// ```rust
/// use adapter::Adapter;
///
/// use mlua::prelude::*;
///
/// pub struct MLuaAdapter(pub Lua);
///
/// impl MLuaAdapter {
///     pub fn new() -> MLuaAdapter {
///         MLuaAdapter(Lua::new())
///     }
///
///     pub fn from_lua(lua: Lua) -> MLuaAdapter {
///         MLuaAdapter(lua)
///     }
/// }
///
/// impl<'lua, Input, Output, Identifier> Adapter<'lua, Input, Output, Identifier> for MLuaAdapter
/// where
///     Input: IntoLuaMulti<'lua>,
///     Output: FromLuaMulti<'lua>,
///     Identifier: IntoLua<'lua>,
/// {
///     type Error = mlua::Error;
///
///     fn call(&'lua mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
///         let lua = &self.0;
///         let globals = lua.globals();
///         let func: mlua::Function = globals.get(identifier)?;
///         func.call::<Input, Output>(input)
///     }
/// }
/// ```
pub trait Adapter<'a, Input, Output, Identifier> {
    /// Errors produced by the plugin provider.
    type Error;

    /// Process a call and return the response synchronously.
    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error>;
}
