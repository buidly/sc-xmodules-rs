# Usage

xModules is a library written in Rustlang that behaves like a crate. For all you entry level builders out there we will
leave here a link to Rustlang documentation about [crates/packages](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html).


## Where to use xModules


xModules will work under the umbrella of your contact, so the first step for this would be to have a base contract,
that is also defined by a crate, and add as a dependency the `xmodules` crate as follows:

*your_contract/Cargo.toml*
```toml
# Please refer to the crates.io page for finding the latest version
# https://crates.io/crates/xmodules
[depdendencies.xmodules]
version = "0.x.x"
```

## How to add to your contract

xModules can be a vast library of modules so please first check the library code to see which module you would like to
use and its capabilities.

Once you know which module to use simply import (or `use`) the crate and module in the
contract root and inherit it over the main contract.


*your_contract/lib.rs*
```rust
use xmodules::my_module;

#[elrond_wasm::contract]
pub trait MyContract: my_module::MyModule {
    ...
```

Right now all the functions, storages and requires are added to your contract and can freely use it inside it as usual.

!!! tip "Extending modules"
    Beside using the modules directly on a contract, you can also extend another module by using any xmodule you find
    suitable for your case.
    ```rust hl_lines="3"
        use xmodules::my_module;

        #[elrond_wasm::module]
        pub trait MyContractModule: my_module::MyModule {
            ...
    ```