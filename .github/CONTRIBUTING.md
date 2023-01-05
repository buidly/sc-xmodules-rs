# Contributing to xModules

Welcome to the contributing guidelines for __xModules__! We appreciate any and all contributions and are grateful for
your efforts to improve our project. In order to streamline the contribution process and make it as easy as possible for
you to help out, we have put together the following guidelines. Please take a moment to read through them before
you begin.

We welcome contributions in the form of:
- code
- documentation
- bug reports
- feature requests

Note: If you are unsure about something, please don't hesitate to ask. We are here to help and want to make sure that
your contributions are accepted and merged as smoothly as possible.

Thank you again for your interest in contributing to our project. We look forward to working with you!

## Implementing a new module

For all the engineers and builders out there that what to implement a new module we assembled a detailed guide for you
to follow. We start with the following steps:

### Step 1: Adding your code

The first step is to implement a rust trait that defines the functions and storage your module will contain that will be
added to a contract. For this example the name `MyModule` will be used to define the module we are trying to implement.

Start by creating a new file inside the `xmodules` crate with the name `my_module.rs` and add it to the library root,
meaning importing `my_module.rs` in `lib.rs`

*lib.rs*
```rust
#![no_std]
...
pub mod my_module;
```

In `my_module.rs` implement the `MyModule` trait bt defined the trait with the module macro attribute to the trait
`#[elrond_wasm::module]`

*my_module.rs*
```rust
#[elrond_wasm::module]
pub trait MyNewModule {
    ...
```

In this trait you add all the logic you want inside the module.
> Note: Try as best you can to decouple it from other modules to overcome coupling and easily extending and implementing
this module in other contracts.

### Step 2: Create tests

For testing our modules we already implemented an empty contract with the full purpose of testing the modules inside the
`xmodules` crate. This empty contract should not contain any logic but only inherit the modules traits.

To start testing your module you first need to create a contract interaction file in order to have the functions for
interacting with the logic you added in the module. Since we don't want to keep all the interactions from all the
modules together we have separate test setup files inside the tests contract.

In the `tests/setup` folder of the tests contract create a new file with the same name as the module you added in the
`xmodules` crate. For this example this will be called `setup/my_module.rs`. Now import the base setup class
`TestsSetup` and create a new implementation.

*tests/setup/my_module.rs*
```rust
/// Importing the base struct
use super::base::TestsSetup;

/// Creating the implementation
impl<Builder> TestsSetup<Builder>
where
    Builder: 'static + Copy + Fn() -> tests::ContractObj<DebugApi>
{
    /// Add your contract interactions here
}
```

Add your contract interactions to this implementation and everything that is necessary for you to interact with the
module.

> NOTE: Check out the base struct for the base implementation that also contains some helper methods.

Now that you created the contract interactions, you also need to create the tests. Create a new file in `tests` using
the prefix test and module name, in this case create the file `tests/test_my_module.rs`. In this file import the tests
setup that will automatically include your contract interactions to use in the tests.

*tests/test_my_module.rs*
```rust
mod setup;
use setup::TestsSetup;

#[test]
fn test_my_module_one() {

    /// This is pretty standard for all tests as it initializes
    /// the base tests setup
    let mut setup = TestsSetup::init(tests::contract_obj);

    /// Add more logic to test your code
}
```

If you don't want to run the entire test suite you can always use the tests feature of cargo and specify which module to
test

```bash
cargo test --test test_my_module
```

To run the entire test suite you can run `cargo test` after the test setup contract is built.

### Step 3. Create a PR to main

From a separate branch using the prefix `feat/my-module-implementation` create a Pull Request to the main branch and
be sure to add description and details about how your module works and why you think it can be beneficial inside the
ecosystem. Maybe throw some examples on were it can be used for other builders to understand this module role


