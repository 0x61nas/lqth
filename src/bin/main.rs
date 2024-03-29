use std::io;

use lqth::{DpyAddr, LqthConfig, Mode, TickTick, Window};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const HELP: &str = r#"
    Options:
      -d, --display <X servevr address>
      -w, --window <window id>
      -s, --screen <screan number>   Specify a screen number (afective when there's no selected window)
      -r, --region <coordinates>     Specify a region to take a screenshot for.
      -v, --version                  Show version information
      -h, --help
"#;

fn main() {
    let mut buf = io::stdout().lock();
    // tick(&mut out_buf, &()).unwrap();
    if let Err(e) = parse_args().tick(&mut buf) {
        fail(format!("{e}").leak());
    }
}

fn parse_args() -> TickTick {
    let mut opts = TickTick::default();
    let mut args = std::env::args().skip(1);

    macro_rules! err {
        ($arg:ident) => {
            fail(format!("Expected one argument after {}, found 0.", $arg).leak());
        };
        (parse; $item:expr, $error:ident) => {
            fail(format!("Can't parse the provided {}, because `{}`", $item, $error).leak())
        };
    }

    macro_rules! value {
            ($from: ident, $arg:ident) => {
                if let Some(v) = $from.next() {
                    v
                } else {
                    err!($arg);
                }
            };
            (parse; $name:expr, $arg:ident) => {
                match value!(args,$arg).parse() {
                    Ok(v) => v,
                    Err(e) => err!(parse; $name, e),
                }
            };
            (parse; $from:ident, $name:expr, $arg:ident) => {
                match value!($from, $arg).parse() {
                    Ok(v) => v,
                    Err(e) => err!(parse; $name, e),
                }
            }
        }

    const WIN_SELECTED: u8 = 0b1;

    let mut flags = 0u8;

    while let Some(arg) = args.next() {
        if !arg.starts_with('-') {
            fail(format!("Unspexted argument {arg}. All the options should start with `-`").leak());
        }
        match arg.trim_start_matches('-') {
            "d" | "addr" | "display" => {
                opts.dpy_addr = DpyAddr::Custom(value!(args, arg).to_owned())
            }
            "w" | "win" | "window" => {
                opts.win = Window::Custom(value!(parse; "window id", arg));
                flags |= WIN_SELECTED;
            }
            "s" | "screen" => {
                if flags & WIN_SELECTED != WIN_SELECTED {
                    opts.win = Window::Root(value!(parse; "screen number", arg))
                }
            }
            "r" | "region" => {
                let mut x = 0;
                let mut y = 0;
                let mut w = 0;
                let mut h = 0;

                let value = value!(args, arg);

                for a in value.split(',') {
                    let Some((label, value)) = a.split_once(':') else {
                        fail("You must provide the labels")
                    };
                    let value = value.trim();
                    match label.trim() {
                        "x" => {
                            x = unsafe {
                                value
                                    .parse()
                                    .map_err(|_e| err!(parse; "X coordinates", a))
                                    .unwrap_unchecked()
                            }
                        }
                        "y" => {
                            y = unsafe {
                                value
                                    .parse()
                                    .map_err(|_e| err!(parse; "Y coordinates", a))
                                    .unwrap_unchecked()
                            }
                        }
                        "w" => {
                            w = unsafe {
                                value
                                    .parse()
                                    .map_err(|_e| err!(parse; "Width", a))
                                    .unwrap_unchecked()
                            }
                        }
                        "h" => {
                            h = unsafe {
                                value
                                    .parse()
                                    .map_err(|_e| err!(parse; "Heghit", a))
                                    .unwrap_unchecked()
                            }
                        }
                        unknown => fail(format!("Error at {unknown}").leak()),
                    }

                    opts.mode = Mode::Selection {
                        start: Some((x, y)),
                        end: Some((w, h)),
                    };
                }
            }
            "v" | "version" => info(format!("{NAME} {VERSION}").leak()),
            "h" | "help" => info(format!("Usage: {NAME} [OPTIONS]\n{HELP}").leak()),
            unknown => fail(format!("Unknown argument `{unknown}`").leak()),
        }
    }
    opts
}

#[cold]
fn fail(msg: &'static str) -> ! {
    eprintln!("{msg}");
    std::process::exit(1)
}

#[cold]
fn info(msg: &'static str) -> ! {
    println!("{msg}");
    std::process::exit(0)
}
