# Spux

Spux is a handcrafted and opinionated library of minimal loaders for Leptos.

[![Crates.io](https://crates.io/crates/spux)
[![Documentation](https://docs.rs/spux/badge.svg)](https://docs.rs/spux)

## Preview



## Getting started

First install Leptos from [leptos.dev](https://leptos.dev)

Then start a new Leptos project with either of the following commands:
Axum: `cargo leptos new --git https://github.com/leptos-rs/start-axum`
Actix Web: `cargo leptos new --git https://github.com/leptos-rs/start-actix` 

Lastly install and add Spux at the root of the new project
```
cargo add spux
```

Once Spux is installed, include the pulser or spinner that you want to use
```rust
use leptos::prelude::*;
use spux::pulsers::Circle;

#[component]
fn App() -> IntoView {

    view! {
        <Circle color="#000000" size=10 />
    }
}
```

Each Spux component takes in required props for both `color` (#hex) and `size` (by px).

| Prop    | Type |
| :------ | :--- |
| color   | &str |
| size    | u32  |

```rust
use leptos::prelude::*;
use spux::pulsers::Diamond;

#[component]
fn App() -> IntoView {

    view! {
        <Diamond color="#000000" size=10 />
    }
}
```

Spux components can also be used with `Suspense` in Leptos.
```rust
use leptos::prelude::*;
use spux::spinners::FilledSquare;

#[component]
fn App() -> IntoView {

  // posts_view consists of a server function that's being called to
  // return a list of posts. see examples/basic-spinner for more details
  view! {
    <Suspense fallback=move || view! {
        <div
            style:width="full"
            style:margin-x="auto"
            style:align-items="center"
            style:justify-content="center"
            style:display="flex"
        >
            <FilledSquare color="#000000" size=10 />
        </div>
    }>
        <div>
            <p>"Posts"</p>
            <hr />
            {posts_view}
        </div>
    </Suspense>
  
  }
}
```

Spux loaders are separated into `spinners` and `pulsers`. v0.1.0 comes packed with:
`spinners`::
* `Square`
* `Triangle`
* `FilledSquare`
* `PartialCircle`

`pulsers`::
* `Circle`
* `Diamond`
* `FilledCircle`
* `FilledDiamond`

To use the various `spinners` and `pulsers`, enable them via features in your `Cargo.toml`:
```toml
[dependencies]
spux = { version = "0.1.0", features = ["spinners", "pulsers"] }
```
