<p align="center">
    <img alt="Lqth logo" src="./lqth.png" width="250">
</p>

{{readme}}

# Usage as a flake

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/0x61nas/lqth/badge)](https://flakehub.com/flake/0x61nas/lqth)

Add lqth to your `flake.nix`:

```nix
{
  inputs.lqth.url = "https://flakehub.com/f/0x61nas/lqth/*.tar.gz";

  outputs = { self, lqth }: {
    # Use in your outputs
  };
}

```


## Dependencies graph

![deps graph](./_deps.png)

> Generated with [cargo-depgraph](https://crates.io/crates/cargo-depgraph)

Current version: {{version}}
