# lqth

**Lqth (لقطه) which means "shot" or/and "capture" in Arabic** is a simple but blazingly fast screenshot utility
inspired by [xscreenshot](https://git.codemadness.org/xscreenshot) and follows the suckless philosophy...(AHM, ahmmmm)... most of it :).

[![crates.io](https://img.shields.io/crates/v/lqth.svg)](https://crates.io/crates/lqth)
[![docs.rs](https://docs.rs/lqth/badge.svg)](https://docs.rs/lqth)
[![downloads](https://img.shields.io/crates/d/lqth.svg)](https://crates.io/crates/lqth)
[![license](https://img.shields.io/crates/l/lqth.svg)](https://github.com/0x61nas/lqth/blob/aurora/LICENSE)

## Examples
The boring way:
```rust
use lqth::*;

let tick = TickTick {
   dpy_addr: DpyAddr::Current,
   win: Window::Root(0),
   mode: Mode::Full,
};

let mut buf = Vec::new();
tick.tick(&mut buf).unwrap();
```

Just take a screenshot for the full screen!
```rust
use lqth::LqthConfig;
let mut out = std::io::stdout().lock();
// ok!
().tick(&mut out).unwrap();
```

## The  binary?
> Nah, am a norme person and I don't wanna write code in this language to JUST TAKE A SCREENSHOT, can you give me a JW solution?

We offer a simple binary that's implement the most of this crate features. You can build it with the build command or if u use cargo then you can install it via `cargo install lqth`.

> **Note** for more information about the binary and how to use it, you can run `lqth -h` or see this [document](./docs/bin.md).

## Wayland?
Nah, I luv my X.


## Contributing
I'm happy to accept any contributions, just consider reading the [CONTRIBUTING.md](https://github.com/0x61nas/lqth/blob/aurora/CONTRIBUTING.md) guide first. to avoid waste waste our time on some unnecessary things.

> the main keywords are: **signed commits**, **conventional commits**, **no emojis**, **linear history**, **the PR shouldn't have more than tree commits most of the time**

## License
This project is licensed under [MIT license][mit].

[mit]: https://github.com/0x61nas/lqth/blob/aurora/LICENSE



License: MIT
