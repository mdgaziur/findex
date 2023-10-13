# Findex Plugins System Guide
This guide explains the findex plugin system to the users and developers.

## For users
- Download the plugin. If it's not compiled, then compile it in release mode using `cargo build --release`
- Go to `target/release` folder.
- You'll see a file with `.so` extension. Copy that to a convenient place.
- Add the following line to your `settings.toml` and edit it to make findex use the plugin:
```toml
PLUGIN = { path = "PLUGIN PATH", prefix = "OPTIONAL USER DEFINED PREFIX", keyboard_shortcut = "OPTIONAL USER DEFINED KEYBOARD SHORTCUT", config = {} }
```

| Property          | Description                                                                                                                                   |
|-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| path              | Path to the plugin's `.so` file                                                                                                               |
| prefix            | Optional user defined prefix that'll make findex use this instead of the one specified in the plugin.                                         |
| config            | Plugin's configuration. Please refer to the plugin's documentation for more information.                                                      |
| keyboard_shortcut | Optional custom keyboard shortcut for triggering the plugin(inserts the plugins prefix into the search box). Must be a valid Gtk accelerator. |
- For more information, please refer to the documentation of the plugin you are using.

**NOTE**: always make sure that the plugins you are using are compatible with the version of Findex you are using.

## For developers

Only Rust based plugins are supported.

- First make a `cdylib` library
- Add `findex-plugin` and `abi_stable` as dependency
- Add the following code into `src/lib.rs`
```rust
use findex_plugin::{define_plugin, FResult};
use abi_stable::std_types::*;

fn init(config: &RHashMap<RString, RString>) -> RResult<(), RString>  {
    // Set up your plugin using the config if necessary
    // Return RErr if something went wrong
    
    // Returning this indicates that the plugin initialization is successful
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let mut result = vec![];
    
    /* Do stuff here */
    
    RVec::from(result)
}

define_plugin!("prefix!", init, handle_query);
// or add the following if you want to have custom shortcut for triggering the plugin.
// The following sets the shortcut to Ctrl+Shift+P.
// define_plugin!("prefix!", "<Ctrl><Shift>P", init, handle_query);
```
- Edit this to create your plugin.
- After writing code, follow user guide to test your plugin

### Explanation

#### Prefix
This is used to invoke the plugin's query handler. This is the first argument of the `define_plugin!` macro. User can
overwrite this by providing a custom prefix like following:
```toml
PLUGIN = { prefix = "custom_prefix!", path = "plugin_path", config = {} }
```

#### Keyboard shortcut
This is used to invoke the plugin's query handler using shortcut keys. This is the second argument of the `define_plugin!`
macro. Under the hood, Findex inserts the prefix for the plugin into the search box when it's pressed.
```toml
PLUGIN = { keyboard_shortcut = "<Ctrl><Shift>P", path = "plugin_path", config = {} }
```

#### `init` function
The `init` function is the function that Findex calls during startup. Plugins may use this to do necessary initialization.
Plugins that do not need any initialization can just return without doing anything. The first argument of the function is 
plugin specific configuration.

The user may provide configuration in the following format:
```toml
PLUGIN = { path = "plugin_path", config = { key1 = "value1", key2 = "value2" } }
```

As you can see, every key will have a string value. This function is the third argument of the `define_plugin!` macro.

#### `handle_query` function
This function gets called every time a user invokes the plugin by typing the prefix. The first argument is the query the 
user typed after the prefix. The function is expected to return a RVec containing results(if any). This function is the 
fourth argument of the `define_plugin!` macro.
