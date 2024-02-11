# error_qa

There is a simply Q&A web application, create by Leptos with Rust

[Leptos](https://leptos.dev/)

## Prerequisite

You should install Rust toolchains and up-to-date

Leptos CSR development tools:

1.  "Trunk" for running leptos CSR site
```
cargo install trunk
```

2. Add `wasm-unknown-unknown` target for that Rust can compile Rust code to WebAssembly to run in browser
```
rustup target add wasm32-unknown-unknown
```

3. Make sure you have added 'npm' cause we used `Tailwindcss` and `DaisyUI`

## Start

Now run 

```
npm i
```

from the root of the project, and run

```
trunk serve --open
``` 

from the root of the project directory, and you will see `Trunk` open the website in you default browser

## build

Run
```
trunk build --release
```
will build all file your need in dist directory at root