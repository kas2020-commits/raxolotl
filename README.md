## Raxolotl

Raxolotl is an ongoing project which aims to provide a monolothic library for
students to easily use simple implementations of common numerical methods.
Currently here some highlights:

- Fast Fourier Transform (and its inverse)
- Root finding algorithms
- ODE linspace solvers
-

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
can also add this line in your code:
```
use raxolotl::*;
```

Probably not the best idea though considering you would be heavily cluttering
the global namespace

In your `Cargo.toml` file, you need to include it as a dependancy:

```
[dependencies]
raxolotl = { git = "https://github.com/kas2020-commits/raxolotl", branch = "master" }
```
