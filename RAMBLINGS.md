
# Plugin System Testing

Idea: Is it possible to create a generic plugin system using some standard interface.

This would need to be able to communicate through some sort of FFI (e.g. rlua) or through other processes / binaries e.g. WASM

We would need to be able to define some sort of standard interface like LSP does.

LSP uses JSON RPC, but we could use any format.

Essentially, do we want to define a method of communicating or simply load a library.

Alternatively, if we are going to design a basic ABI, do we want it interoperable some other schema.

e.g.

Passing a message via an ABI would be the same as via a socket or JSON RPC 

```rust

struct Message {
    content: serde::Value, // Maybe serde_json?
    type: TypeEnum, // type of message?
}

```

[This blog](https://nullderef.com/blog/plugin-tech/) outlines a few good methods to be able to pass data through with a plugin system.

### The Plan:
- Implement some basic generic types that allow us to pass data to our system.
- We want to be able to offer a lot of backends for the system, with the same front end. UI.

```rust
use plugin_system::{ PluginSystem, LuaAdapter };

struct Message {
  content: serde::Value, // Maybe serde_json?
  type: TypeEnum, // type of message?
}

fn main(){

  let adapter = LuaAdapter::new();

  adapter.load(some_path);

  let system = PluginSystem::new(adapter);

  PluginSystem.listen<Input=Message>();

}

```


## Need to formally define

- What is a plugin?
- How does a plugin interact with a piece of software?
- How can we make it extensible?
- Do we we want to define a standard such as OpenAPI or Protobuf to define communication between the plugin and the software?
- Do we want to sandbox the plugins system if they are WASM?
  - Do we give them access to the filesystem?
  - Do we give them access to the network?



A plugin can:
- Listen for events
- Send Events
  - How do we listen to events?
  - How do we send events?
- Run Commands
  - A command should be a function that can be called
  - Or is a command defined on the software's side?
    - Finite set of commands defined that the plugin can run?
- Access the filesystem


How does ZelliJ do it?
- They have a protobuf file that determines the API that any plugin can use
- Changing this file updates the API.
- Not sure if they use bindgen to generate any code/bindings for that?
- They define a list of functions in the code that they can interact with.
- They then expose that for the end-user to use.

SO:
- User should be able to define list of functions that a plugin can use.
- User should be able to define a list of events that a plugin can listen to.
- User should be able to define a list of events that a plugin can send.

How can we define this effectively?
- We could use a protobuf file to define the API.
- We could use a rust file to define the API.
  - This would mean that the user would have to compile the rust file first to "generate" bindings

- Follow the LSP standard and use something like JSON RPC to send messages
- Could define custom messages on user side.
 - For WASM, could then send messages via socket



```filetree
|- src
|  |- lib.rs // impl
|  |- events.rs // define events
|  |- commands.rs // define commands
```

Then when we load the plugin, we can load a list of functions and commands that we call and call them through that.

What is an adapter?
- An adapter is a way to define a plugin system for a specific interface / language / runtime.
- An adapter should be able to take in the current API, and ensure that the plugin is following that API.


THIS IS A RUST FOCUSED LIBRARY, RUST IS PRIORITY.


Step 1:
What do we want to be able to do?
- Take in some route to a plugin.
- List the available functions in that plugin.


Step 2:
Define an API for the plugin.
- This should be a list of functions that the plugin can call.
- This should be a list of events that the plugin can listen to.
- This should be a list of events that the plugin can send.

Potential to make a signal based system?
Potential to make a Message based system?
Are these the same things?
Does a message based system require some sort of API on the plugin side?
- If we want to use a platform independent system, then not having that API is not really what we want.

WE DO NOT NEED TO THINK ABOUT HOW A PLUGIN INTERACTS WITH THE SOFTWARE. THAT IS THE JOB OF WHOEVER IMPLEMENTS THIS LIBRARY
- well we should think about it but it is not out concern.


- How can a plugin store state?
- If we only use a plugin-based system, then plugins could (in theory) be stateless.
- If we need some sort of state in the plugin, then we need to be able to store that state somewhere.
- If we have a seperate running process, then we can store state in that process.
- Can we store state in WASM plugins?

```rust
// This is the Zellij implementation of a messsage type.
// It keeps track of the worker that it needs to be sent to.
// This is because it loads the WASM as a binary running seperately.
// I do not think that this is the most optimal method, but it is along the right lines.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub worker_name: ::core::option::Option<::prost::alloc::string::String>,
}
```

```rust
// List of events in Zellij
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNameList {
    #[prost(enumeration = "EventType", repeated, tag = "1")]
    pub event_types: ::prost::alloc::vec::Vec<i32>,
}
```

```rust
// Command in Zellij
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub cwd: ::core::option::Option<::prost::alloc::string::String>,
}
```


```rust
// optional payload
// this seems a little strange, but I guess it makes sense
// This is the Zellij implementation of an action
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(enumeration = "ActionName", tag = "1")]
    pub name: i32,
    #[prost(
        oneof = "action::OptionalPayload",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46"
    )]
    pub optional_payload: ::core::option::Option<action::OptionalPayload>,
}
```




Idea:
- If we decide to run the plugins on a seperate process, we can essentially run all code as a seperate process, making abstraction a lot easier.
- We might not want this however.
- We might just want to be able to call some functions in our library.



Another idea 23/01/2024
- All a plugin is really is a way to communicate...
- So just define those ways to communicate
- What if we defined a standard (few methods of communication as well?)
- E.g. 
    - We define a JSON interface between two processes that directly calls
    - We define a socket interface for communicating via sockets
    - We define a ABI interface for interfaces that want to communicate via ABI.
    - For example we may want to combine sockets + JSON for super easy communication
    - Or maybe we use sockets and a custom message format?
    - All we want is to be able to pass data 
    - E.g. We have some connection and we can listen to events on that connection.
    - For an ABI: we define some standard interface / function that we need to deal with
    - For a socket: we can also define some standard messaging format.
    - For WASM: We could do either
        - Have a WASM runtime that receives messages or
        - Have a WASM runtime that we can call functions on with some standard interface 


2 System problem:
- Messaging system?
- API system?


Define Event, Command, Action
- Event is an event on the server side
- Command is server sending command to client
- Action is servers-side equivelant of command (e.g. server does something because client asked it to)
- Message is client sending server an event (e.g. listen to event or something) 


Event + Message = Communicaton system

Command + Action = ABI type system





Or perhaps follow in Serde's footsteps?

Split out code into 3 parts.

- The plugin consumer
    - Application that implements plugins
- The plugin interface
    - Middleware of sorts that acts kind of like serde's data model
- The plugin 
    - The plugin that must interact with the interface

- Potential:
    - Use a tower::Service?
        - Default async (good for multi-paradigm)
        - Allows a Request -> Response solution
        - Large ecosystem already so people will know how to implement

If a plugin is calling stuff directly:
- How do we use a tower::Service?
- It would be as simple how do we get a request?


- Each function on the server side is a tower::Service, that returns the type and data that we require



Think of the plugin service as middleware? Translating between the user side and the
- Like serde?
- A server requests a function to be called on the user side?
    - Calls a tower service, pass in params and call function
- A client requests a function to be called on the user side?
    - Calls a tower service, with params, and call function.

It can be up to the dev how to send these messages across.

If this is the case, A plugin service may need:
- It's own thread
- The type of plugin that it is working with
    - WASM
    - Lua
    - Etc.

- Perhaps it may also be a good idea to go with a plugin registry type structure?

```rust

    let registry = PluginRegistry::new();
    registry.add(plugin, adapter);

```

```rust

struct WasmAdapter<PluginDefintion>;

impl Adapter<PluginDefinition> for WasmAdapter<PluginDefinition> {

}

fn main(){

    let adapter: LuaAdapter = LuaAdapter::new();
    let adapter: WasmAdapter = WasmAdapter::new();

    // Maybe let the lifetime of the plugin be handled by the user?
    let plugin = Plugin::load('path to some file', adapter);


}

```


Perhaps it may also be possible to have a derive based syntax?

#[derive(State)]
#[derive(Plugin)]
#[derive(plugin_type = type)]
#[derive(Adapter)]

Note: We do not want the user to have to deal with settings up the middleware of sorts.

Or perhaps:
- The dev is responsible for settings up their adapter
- The adapter in this case would BE the middleware.
- This is good, as it allows a standard interface to be defined by the user
- Downside is that it means that you can't have multiple adapters for one "system"
    - To bypass this you could just have multiple "systems" running.
    - Most plugins will most likely not have multiple plugin systems


An adapter can hold state:
- The adapter can hold it's own state (FFI type)

- Or let the program/plugin handle it's own state (Messenger type)




# Types of plugin

## FFI

How do we communicate
- ONLY the server can initiate communication (backwards I know but we need to initiate communication somehow)
- Adapter should define a set of methods to communicate for the service
    - This could be:
        - Loading a file (lua files can return data)
        - calling a function
        - straight loading data from a configuration file
    - Then, given an adapter, and some source, the server could easily run client code
    - ```rust

        let adapter: Adapter = MyAdapter::new();

        let plugin = Adapter::load('some_file');

        Adapter::call(plugin, 'some_func_name');

        Plugin::call('some_func_name'); // under the hood calling Adpater::call?
        // This is great! as this is the tower::Service 

        // might wanna abstract this out a bit 
        // Perhaps even cover it up with some wrapper types```
## Messaging


- For the message-type of system, I am thinking of either a LSP type protocol, or we go lower level
- How do we communicate
- A server must initiate communicating 
  - This could be
    - Sending a message
    - Reacting to an Event
    - Doing an Action
    - Sending a Command


```rust

struct LuaAdapter{

};

impl Adapter for LuaAdapter {
    type Response;
    type Error;
    type Future: Future
    where
        <Self::Future as Future>::Output == Result<Self::Response, Self::Error>;

    fn poll_ready(
        &mut self, 
        cx: &mut Context<'_>
    ) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;

    fn load(&mut self, path: std::path::PathBuf) -> Self::Future;

}

struct LuaPlugin {
  some_state: String,
  some_other_state: i32,
};

impl Plugin<LuaAdapter> for LuaPlugin {
}
```




A plugin should:
- Define the set of functions that it can call
- Define the events that it can listen to
- Define the events that it can send


```rust
trait Plugin<Adapter> {
  type Adapter;
  type Error;
  type Response;
  type Events;
  type Commands;
}

struct LuaAdapter;

impl Adapter for LuaAdapter {
  type Error;
  type Response;
  type Commands;
}



impl Plugin for LuaPlugin {
 
}
```


What do we want to define for a Plugin

```rust
Plugin::call("function", args);

// Perhaps
Plugin::call(Cmd::new("function", args));

// We must define: How a call works
// How do we define a call?
```



We need well defined split between the client (program), adapter, and the server (plugin).
- Do we want to use a tower::Service?



An adapter should be able to:
- Load a plugin
- Call a function on a plugin
- Unload a plugin







One way of thinking:
- Adapter is purely the communication layer
- Client side plugin handles the event system, command system etc.


a event maps events to commands on the plugin side
a message maps messages to actions on the client side




Example Lua Adapter:

```rust
use crate::Adapter;

use mlua::{Lua, Result};

struct LuaAdapter {
  lua: Lua,
};

impl Adapter for LuaAdapter {

  fn new() -> Self {
    let lua = Lua::new();
    Self { lua }
  }

  fn load(&mut self, path: &str) -> Result<(), Error> {
    // Load a lua file
    let path = std::path::Path::new(path)?;
    let mut file = std::fs::File::open(path)?;

    self.lua.context(|lua_ctx| {
      lua_ctx.load(file)
    })?;

    Ok(())
  }

  fn call<'lua, T>(ctx: Context<'lua>, func_name: &str, param: T) -> Result<()>
  where
    T: ToLua<'lua>,
  {
    let func: Function = ctx.globals().get(func_name)?;
    func.call::<T, ()>(param)
  }

  fn _call<T>(&mut self, func_name: &str, param: T) -> Result<()>
  where
      T: for<'a> ToLua<'a>,
  {
      self.lua.context(|context| {
          let func: Function = context.globals().get(func_name)?;
          func.call::<T, ()>(param)
      })
  }

}





struct LuaPlugin;

// Where T is our message type
trait Plugin<A, M>
where
  A: Adapter + Service<M>
{
  
} 

```






```rust
use mlua::{Lua, Function, Context, ToLua, Result};
use std::future::Future;
use tower::Service;
use serde_json::Value as JsonValue;

// Define the Adapter trait without the call method
pub trait Adapter {
    fn new() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
    fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
    // You can add other methods specific to the Adapter trait
}

// Define the LuaAdapter struct
pub struct LuaAdapter {
    lua: Lua,
}

// Implement the Adapter trait for LuaAdapter
impl Adapter for LuaAdapter {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path);
        let mut file = std::fs::File::open(path)?;

        self.lua.context(|lua_ctx| {
            lua_ctx.load(&mut file)
        })?;

        Ok(())
    }
}

// Implement the Service trait for LuaAdapter
impl Service<String> for LuaAdapter {
    type Response = JsonValue;
    type Error = Box<dyn std::error::Error>;
    type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        // This method is used to check if the service is ready to process requests.
        // You can customize it based on your needs.
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: String) -> Self::Future {
        // This method processes incoming requests and returns a Future representing the result.
        let lua = self.lua.clone();
        Box::pin(async move {
            lua.context(|context| {
                let func: Function = context.globals().get(&request)?;
                let result: Result<JsonValue> = func.call(());
                result.map_err(Into::into)
            })
        })
    }
}
```


```rust

trait Adapter {
    // serialize your data here
    send(){}

    // deserialize your data here
    receive(){}

}

struct Cmd {
    a: A + FromBytes,
    b: ToBytes
}

```




ONLY EVENT AND COMMANDS OOPSSSSSS
