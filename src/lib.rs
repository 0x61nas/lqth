//! **Lqth (لقطه) which means "shot" or/and "capture" in Arabic** is a simple but blazingly fast screenshot utility
//! inspired by [xscreenshot](https://git.codemadness.org/xscreenshot) and follows the suckless philosophy...(AHM, ahmmmm)... most of it :).
//!
//! [![crates.io](https://img.shields.io/crates/v/lqth.svg)](https://crates.io/crates/lqth)
//! [![docs.rs](https://docs.rs/lqth/badge.svg)](https://docs.rs/lqth)
//! [![downloads](https://img.shields.io/crates/d/lqth.svg)](https://crates.io/crates/lqth)
//! [![license](https://img.shields.io/crates/l/lqth.svg)](https://github.com/0x61nas/lqth/blob/aurora/LICENSE)
//!
//! # Examples
//! The boring way:
//! ```no_run
//! use lqth::*;
//!
//! let tick = TickTick {
//!    dpy_addr: DpyAddr::Current,
//!    win: Window::Root(0),
//!    mode: Mode::Full,
//! };
//!
//! let mut buf = Vec::new();
//! tick.tick(&mut buf).unwrap();
//! ```
//!
//! Just take a screenshot for the full screen!
//! ```no_run
//! use lqth::LqthConfig;
//! let mut out = std::io::stdout().lock();
//! // ok!
//! ().tick(&mut out).unwrap();
//! ```
//!
//! # The  binary?
//! > Nah, am a norme person and I don't wanna write code in this language to JUST TAKE A SCREENSHOT, can you give me a JW solution?
//!
//! We offer a simple binary that's implement the most of this crate features. You can build it with the build command or if u use cargo then you can install it via `cargo install lqth`.
//!
//! > **Note**
//! > for more information about the binary and how to use it, you can run `lqth -h` or see this [document](./docs/bin.md).
//!
//! # Wayland?
//! Nah, I luv my X.
//!
//!
//! # Contributing
//! I'm happy to accept any contributions, just consider reading the [CONTRIBUTING.md](https://github.com/0x61nas/lqth/blob/aurora/CONTRIBUTING.md) guide first. to avoid waste waste our time on some unnecessary things.
//!
//! > the main keywords are: **signed commits**, **conventional commits**, **no emojis**, **linear history**, **the PR shouldn't have more than tree commits most of the time**
//!
//! # License
//! This project is licensed under [MIT license][mit].
//!
//! [mit]: https://github.com/0x61nas/lqth/blob/aurora/LICENSE
//!
//!

use std::{io::Write, mem::MaybeUninit, ptr};

use byteorder::{BigEndian, ByteOrder};
use x11::xlib::{
    XCloseDisplay, XDestroyImage, XGetImage, XGetPixel, XGetWindowAttributes, XGrabServer,
    XOpenDisplay, XRootWindow, XUngrabServer, XWindowAttributes, ZPixmap, _XDisplay,
};

const MAGIC_BYTES: &[u8; 8] = b"farbfeld";
const ALPHA_BYTES: &[u8; 2] = &u16::MAX.to_be_bytes();
const ALL_PLANES: u64 = !0; // a.k.a. 0xffff_ffff

#[derive(thiserror::Error, Debug)]
pub enum TickError {
    #[error("Can't open the selected X display")]
    CantOpenDpy,
    #[error("Can't get an image for the selected window")]
    CantGetImage,
    #[error("{0}")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TickError>;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DpyAddr {
    Custom(String),
    #[default]
    Current,
}

impl DpyAddr {
    fn ptr(&self) -> *const i8 {
        match self {
            DpyAddr::Custom(addr) => addr.as_ptr().cast(),
            DpyAddr::Current => ptr::null(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Window {
    Custom(u64),
    Root(i32),
}

impl Window {
    fn id(&self, dpy: *mut _XDisplay) -> u64 {
        match self {
            Window::Custom(id) => id.to_owned(),
            Window::Root(screen_num) => unsafe { XRootWindow(dpy, screen_num.to_owned()) },
        }
    }
}
pub type Point = (u32, u32);
pub type PointI = (i32, i32);

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Mode {
    #[default]
    Full,
    Selection {
        start: Option<PointI>,
        end: Option<Point>,
    },
}

impl Mode {
    #[inline]
    fn transform(&self, win_attr: XWindowAttributes) -> (PointI, Point) {
        match self {
            Mode::Full => ((0, 0), (win_attr.width as u32, win_attr.height as u32)),
            Mode::Selection { start, end } => (
                start.unwrap_or((0, 0)),
                end.unwrap_or((win_attr.width as u32, win_attr.height as u32)),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TickTick {
    pub dpy_addr: DpyAddr,
    pub win: Window,
    pub mode: Mode,
}

impl Default for TickTick {
    fn default() -> Self {
        Self {
            dpy_addr: DpyAddr::default(),
            win: Window::Root(0),
            mode: Mode::default(),
        }
    }
}

pub trait LqthConfig {
    fn dpy_addr(&self) -> &DpyAddr;
    fn win(&self) -> Window;
    fn mode(&self) -> Mode;

    #[inline(always)]
    fn tick<W: Write>(&self, out_buf: &mut W) -> Result<()> {
        crate::tick(out_buf, self)
    }
}

impl LqthConfig for TickTick {
    #[inline(always)]
    fn dpy_addr(&self) -> &DpyAddr {
        &self.dpy_addr
    }

    #[inline(always)]
    fn win(&self) -> Window {
        self.win.clone()
    }

    #[inline(always)]
    fn mode(&self) -> Mode {
        self.mode.clone()
    }
}

impl LqthConfig for () {
    fn dpy_addr(&self) -> &DpyAddr {
        &DpyAddr::Current
    }

    fn win(&self) -> Window {
        Window::Root(0)
    }

    fn mode(&self) -> Mode {
        Mode::default()
    }
}

/// Take a screenshot based on the config and write it as a farbfeld bytes to the buffer
#[inline]
pub fn tick<W, C>(out_buf: &mut W, config: &C) -> Result<()>
where
    W: Write,
    C: LqthConfig + ?Sized,
{
    let dpy = unsafe { XOpenDisplay(config.dpy_addr().ptr()) };
    if dpy.is_null() {
        return Err(TickError::CantOpenDpy);
    }
    let mut win_attr = MaybeUninit::uninit();
    // SAFETY: we are sure that the `dpy` pointer is valid.
    let win = config.win().id(dpy);
    // TODO: should check on the window id and see if there an window with this id or not?
    unsafe {
        XGrabServer(dpy);
        XGetWindowAttributes(dpy, win, win_attr.as_mut_ptr())
    };
    // SAFETY: in this point, the `win_attr` should be initialized. Otherwise, the X server should be killed the process already.
    let win_attr = unsafe { win_attr.assume_init() };
    let ((xs, ys), (xe, ye)) = config.mode().transform(win_attr);
    let img_ptr = unsafe {
        XGetImage(
            dpy, win, //
            xs, ys, // x and y
            xe, ye, // Width and height
            ALL_PLANES, ZPixmap,
        )
    };
    unsafe {
        XUngrabServer(dpy);
        XCloseDisplay(dpy);
    }
    if img_ptr.is_null() {
        return Err(TickError::CantGetImage);
    }
    let img = unsafe { *img_ptr };

    let sr: u8;
    let sg: u8;
    let fr: u16;
    let fb: u16;
    let fg: u16;

    match img.bits_per_pixel {
        16 => {
            sr = 11;
            sg = 5;
            fr = 2047;
            fb = 2047;
            fg = 1023;
        }
        24 | 32 => {
            sr = 16;
            sg = 8;
            fr = 257;
            fg = 257;
            fb = 257;
        }
        other => panic!("Unsupported bpp: {other}"),
    }

    // The magic value
    out_buf.write_all(MAGIC_BYTES)?;

    let mut buf = [0u8; 4];
    BigEndian::write_u32(&mut buf, img.width as u32);
    out_buf.write_all(&buf)?; // 4 bytes
    BigEndian::write_u32(&mut buf, img.height as u32);
    out_buf.write_all(&buf)?; // 4 bytes

    macro_rules! write_channel {
        ($out: ident; $buf: ident, $cn: expr) => {
            {
                BigEndian::write_u16(&mut $buf, $cn);
                $out.write_all(&$buf)
            }
        };
        ($out: ident; $buf: ident; channels: $($cn: expr,)+) => {
            $(write_channel!($out; $buf, $cn)?;)*
        };
    }

    let mut buf = [0u8; 2];
    // write pixels
    for h in 0..img.height {
        for w in 0..img.width {
            // SAFETY: If we reatch to here, then we're sure that the `img_ptr` are valid. Also `w` and `h` will always be in the renge.
            let p = unsafe { XGetPixel(img_ptr, w, h) };
            write_channel! { out_buf; buf;
                channels:
                ((p & img.red_mask) >> sr) as u16 * fr,
                ((p & img.green_mask) >> sg) as u16 * fg,
                (p & img.blue_mask) as u16 * fb,
            };
            // The alpha channel will always = MAX
            out_buf.write_all(ALPHA_BYTES)?;
        }
    }
    // SAFETY: we are sure that our pointer is valid.
    unsafe { XDestroyImage(img_ptr) };
    Ok(())
}
