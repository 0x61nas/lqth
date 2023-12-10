//! **Lqth (لقطه) which means "shot" or/and "capture" in Arabic** is a simple but blazingly fast screenshot utility
//! inspired by [xscreenshot](https://git.codemadness.org/xscreenshot) and follows the suckless philosophy...(AHM, ahmmmm)... most of it :).
//!
//!

use std::{
    io::{self, Write},
    mem::MaybeUninit,
    ptr,
};

use byteorder::{BigEndian, ByteOrder};
use x11::xlib::{
    XCloseDisplay, XDestroyImage, XGetImage, XGetPixel, XGetWindowAttributes, XGrabServer,
    XOpenDisplay, XRootWindow, XUngrabServer, ZPixmap,
};

const MAGIC_BYTES: &[u8; 8] = b"farbfeld";
const ALPHA_BYTES: &[u8; 2] = &u16::MAX.to_be_bytes();
const ALL_PLANES: u32 = !0; // a.k.a. 0xffff_ffff

/// Take a screenshot for the full screen.
#[inline]
pub fn tick<W: Write>(out_buf: &mut W) -> Result<(), Box<dyn std::error::Error>> {
    let dpy = unsafe { XOpenDisplay(ptr::null()) };
    // dbg!(dpy);
    let win = unsafe { XRootWindow(dpy, 0) };
    unsafe { XGrabServer(dpy) };
    let mut win_attr = MaybeUninit::uninit();
    unsafe { XGetWindowAttributes(dpy, win, win_attr.as_mut_ptr()) };
    let win_attr = unsafe { win_attr.assume_init() };
    let img_ptr = unsafe {
        XGetImage(
            dpy,
            win,
            0,
            0,
            win_attr.width as u32,
            win_attr.height as u32,
            ALL_PLANES.into(),
            ZPixmap,
        )
    };
    if img_ptr.is_null() {
        panic!("XGetImage");
    }
    let img = unsafe { *img_ptr };
    unsafe {
        XUngrabServer(dpy);
        XCloseDisplay(dpy);
    }

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
    BigEndian::write_u32(&mut buf, win_attr.width as u32);
    out_buf.write_all(&buf)?; // 4 bytes
    BigEndian::write_u32(&mut buf, win_attr.height as u32);
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
    for h in 0..win_attr.height {
        for w in 0..win_attr.width {
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
    // out_buf.flush()?;
    unsafe { XDestroyImage(img_ptr) };
    Ok(())
}
