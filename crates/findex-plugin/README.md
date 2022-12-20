# Findex Plugins Guide

## For users
- First download and compile the plugin in release mode. This will compile the plugin to a dynamically linked library.
- Determine the name of the compiled binary. For example, if the package name is "github_repo", then the name will be "libgithub_repo.so"
- The compiled binary will live in "PLUGIN/target/release/libPLUGIN.so"
- Copy that to a convenient place. 
- Now add entry to `settings.toml` in the following format:
```toml
PLUGIN = { path = "PLUGIN PATH", prefix = "OPTIONAL USER DEFINED PREFIX", config = {} }
```
- Refer to the plugin's usage instruction for info about how to use it.

**NOTE**: always make sure that the plugins you are using are compatible with the version of Findex you are using.

## For developers

Only Rust based plugins are supported.

- First make a `cdylib` library
- Add `findex-plugin` and `abi_stable` as dependency
- Add the following code into `src/lib.rs`
```rust
use abi_stable::pmr::RResult;
use findex_plugin::{define_plugin, FResult};
use abi_stable::std_types::{RHashMap, ROk, ROption, RStr, RString, RVec};

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString>  {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    if query == "" {
        return RVec::new();
    }

    RVec::from(vec![FResult {
        cmd: RString::from(format!("xdg-open \"{query}\"")),
        name: RString::from(format!("Open {query}")),
        desc: ROption::RNone,
        score: isize::MAX,
        icon: RString::from("browser"),
    }])
}

define_plugin!("url!", init, handle_query);
```
*This is the code of `urlopen` plugin*
- Edit this to create your plugin.
- After writing code, follow user guide to test your plugin

### Key information
The first argument to define_plugin! macro is the prefix used to call the plugin's query handler. The user can overwrite this.
