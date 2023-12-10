//! **Lqth (لقطه) which means "shot" in Arabic** is a simple but blazingly fast screenshot utility
//! inspired by [xscreenshot](https://git.codemadness.org/xscreenshot) and follows the suckless philosophy

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

/// Take a screenshot for the full screen.
pub fn tick() -> Result<(), Box<dyn std::error::Error>> {
    let dpy = unsafe { XOpenDisplay(ptr::null()) };
    // dbg!(dpy);
    let win = unsafe { XRootWindow(dpy, 0) };
    unsafe { XGrabServer(dpy) };
    let mut win_attr = MaybeUninit::uninit();
    unsafe { XGetWindowAttributes(dpy, win, win_attr.as_mut_ptr()) };
    let win_attr = unsafe { win_attr.assume_init() };
    const ALL_PLANES: u32 = !0; // a.k.a. 0xffff_ffff
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

    let mut out = io::stdout().lock();

    // The magic value
    out.write_all(b"farbfeld")?;

    let mut buf = [0u8; 4];
    BigEndian::write_u32(&mut buf, win_attr.width as u32);
    out.write_all(&buf)?; // 4 bytes
    BigEndian::write_u32(&mut buf, win_attr.height as u32);
    out.write_all(&buf)?; // 4 bytes

    let mut tpix = [0u16; 3];
    let mut buf = [0u8; 2];
    let mut alpha_buf = [0; 2];
    BigEndian::write_u16(&mut alpha_buf, u16::MAX);
    // write pixels
    for h in 0..win_attr.height {
        for w in 0..win_attr.width {
            // let p = img.sequence
            let p = unsafe { XGetPixel(img_ptr, w, h) };
            tpix[0] = ((p & img.red_mask) >> sr) as u16 * fr;
            tpix[1] = ((p & img.green_mask) >> sg) as u16 * fg;
            tpix[2] = (p & img.blue_mask) as u16 * fb;

            for c in tpix {
                BigEndian::write_u16(&mut buf, c);
                out.write_all(&buf)?;
                // for b in buf {
                //     write!(out, "{b}")?;
                // }
            }
            // The alpha channel will always = MAX
            out.write_all(&alpha_buf)?;
        }
    }
    out.flush()?;
    unsafe { XDestroyImage(img_ptr) };
    Ok(())
}
