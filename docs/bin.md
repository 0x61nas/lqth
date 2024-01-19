**lqth** is a simple screenshot utility for X11. It writes thae image in a Farbfeld format to the stdout.

# Build and install

```sh
cargo build -r
```
```sh
  cp target/release/lqth /usr/bin
```

# Dependencies


- libX11


# Optional dependencies

- To convert farbfeld data you can use ff2jpg or ff2png from: https://git.suckless.org/farbfeld/


# Usage examples

Take a screenshot for the full screen

```sh
lqth > screen.ff
```

Take a screenshot for the active window

```sh
lqth -w $(xdotool getactivewindow) | ff2png > window.png
```

Take a screenshot for a spesfic region

```sh
lqth -r "$(xrectsel "x:%x,y:%y,w:%w,h:%h")" | ff2png > region.png
```

Take a screenshot of a specific region and copy it into the system clipboard

```sh
lqth -r $(xrectsel "x:%x,y:%y,w:%w,h:%h") | ff2png | xclip -sel clip -t image/png -i
```

> Yep is just simple as that :)

