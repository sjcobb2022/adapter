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
/// use mlua::{prelude::*, AsChunk};
///
/// struct MLuaAdapter {
///     lua: Lua,
/// }
///
/// impl MLuaAdapter {
///     fn new() -> Self {
///         MLuaAdapter { lua: Lua::new() }
///     }
/// }
///
/// impl<'a, Input, Output, Identifier> Adapter<'a, Input, Output, Identifier> for MLuaAdapter
/// where
///     Input: IntoLuaMulti<'a>,
///     Output: FromLuaMulti<'a>,
///     Identifier: for<'b> AsChunk<'b, 'a>,
/// {
///     type Error = mlua::Error;
///
///     fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error> {
///         self.lua.load(identifier).call::<Input, Output>(input)
///     }
/// }
/// ```
pub trait Adapter<'a, Input, Output, Identifier> {
    /// Errors produced by the plugin provider.
    type Error;

    /// Process a call and return the response synchronously.
    fn call(&'a mut self, identifier: Identifier, input: Input) -> Result<Output, Self::Error>;
}
