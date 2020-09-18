## raxolotl

Raxolotl is an adaptation from the origional
[Axolotl++](https://github.com/kas2020-commits/axolotl) library.

### Installation

- To run the tests implemented in `tests/`, simply run `$ cargo test`.

- To build the library, run `$ cargo build --release`.

### Usage

If you are using this library within rust all you need to do is add this in the
code:

```
extern crate raxolotl;
```

If you want to avoid calling the namespace every time you use the library you
can also add this:
```
use raxolotl::*
```

Considering that the API is not frozen, I do not recommend this unless you check
the source code

In your `Cargo.toml` file, you need to include it as a dependancy:

```
[dependencies]
raxolotl = { git = "https://github.com/kas2020-commits/raxolotl", branch = "master" }
```
