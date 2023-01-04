# Buidly X-Modules


# Abstract
This project aims to create a library with open source contracts that can be used by MultiversX developers. These can be
integrated easily within any smart contract because they behave exactly as any other wasm module.


# Usage
Check out the latest release of the library on `crates.io` by following this link [xmodules crate](https://crates.io/crates/open-modules).
The version is specified in the install instructions or directly add them in your `Cargo.toml` file for the smart contract
with:
```toml
[dependencies.xmodules]
version = "x.x.x"
```


In your smart contract main lib entry inherit the module you want to use directly on the contract trait:
```rust
use xmodules::my_module;

#[elrond_wasm::contract]
pub trait MyContract: my_module::MyModule {
    ...
```

Or use it on another module, but beware the contract also needs to implement the trait


```rust
use xmodules::my_module;

#[elrond_wasm::module]
pub trait MyOtherModule: my_module::MyModule {
    ...

#[elrond_wasm::contract]
pub trait MyContract:
    MyOtherModule +
    my_module::MyModule
{
    ...
```

