use std::io;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

use raw_window_handle::RawWindowHandle;
use x11_dl::xlib::{Display, Visual, XGCValues, XImage, XWindowAttributes, Xlib, ZPixmap, GC};

use crate::{PixelBufferCreationError, PixelBufferFormatSupported, PixelBufferFormatType};

pub struct PixelBuffer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    ximage: *mut XImage,
    display: *mut Display,
    window: c_ulong,
    xlib: Xlib,
    gc: GC,
}

fn get_window_and_display(handle: RawWindowHandle) -> Option<(c_ulong, *mut Display)> {
    if let raw_window_handle::RawWindowHandle::Xlib(x_handle) = handle {
        return Some((x_handle.window, x_handle.display as *mut Display));
    }
    None
}

impl PixelBufferFormatSupported for crate::RGBA {}
impl PixelBufferFormatSupported for crate::RGB {}
pub type NativeFormat = crate::RGBA;

const BYTES_PER_PIXEL: usize = 4;
const BITS_PER_PIXEL: usize = BYTES_PER_PIXEL * 8;

impl PixelBuffer {
    pub unsafe fn new(
        width: u32,
        height: u32,
        format: PixelBufferFormatType,
        raw_window_handle: RawWindowHandle,
    ) -> Result<PixelBuffer, PixelBufferCreationError> {
        // TODO(wathiede): validate format.
        // TODO(wathiede): something with raw_window_handle.
        let x = Xlib::open().expect("failed to open Xlib library");
        let (window, display) =
            get_window_and_display(raw_window_handle).expect("handle wasn't an XlibHandle");
        let pixels = vec![255; (width * height) as usize * BYTES_PER_PIXEL];
        let (depth, visual) = {
            let mut xwa: XWindowAttributes = std::mem::zeroed();
            dbg!((x.XGetWindowAttributes)(display, window, &mut xwa));
            (xwa.depth as c_uint, xwa.visual)
        };
        dbg!(depth, visual);
        let gc = (x.XCreateGC)(display, window, 0, 0 as *mut XGCValues);
        let format = ZPixmap;
        let offset = 0;
        let data = pixels.as_ptr();
        let width = width as c_uint;
        let height = height as c_uint;
        let bitmap_pad = 32;
        let bytes_per_line = 0;
        let ximage = (x.XCreateImage)(
            display,
            visual as *mut Visual,
            depth,
            format,
            offset,
            data as *mut c_char,
            width,
            height,
            bitmap_pad,
            bytes_per_line,
        );
        if ximage.is_null() {
            // TODO(wathiede): better error handling.
            panic!("Couldn't create XImage");
        }
        // TODO(wathiede): cleanup X resources when PixelBuffer is dropped.
        dbg!(&width, &height, &depth, &ximage, &display, &window, &gc,);
        Ok(PixelBuffer {
            width,
            height,
            pixels,
            ximage,
            xlib: x,
            display,
            window,
            gc,
        })
    }
    pub unsafe fn blit(&self, handle: RawWindowHandle) -> io::Result<()> {
        self.blit_rect((0, 0), (0, 0), (self.width(), self.height()), handle)
    }
    pub unsafe fn blit_rect(
        &self,
        src_pos: (u32, u32),
        dst_pos: (u32, u32),
        blit_size: (u32, u32),
        handle: RawWindowHandle,
    ) -> io::Result<()> {
        (self.xlib.XPutImage)(
            self.display,
            self.window,
            self.gc,
            self.ximage,
            src_pos.0 as c_int,
            src_pos.1 as c_int,
            dst_pos.0 as c_int,
            dst_pos.1 as c_int,
            blit_size.0 as c_uint,
            blit_size.1 as c_uint,
        );
        let discard = 0;
        (self.xlib.XSync)(self.display, discard);
        //(self.xlib.XFlush)(self.display);

        Ok(())
    }
    pub fn bits_per_pixel(&self) -> usize {
        BITS_PER_PIXEL
    }

    pub fn bytes_per_pixel(&self) -> usize {
        BYTES_PER_PIXEL
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn row_len(&self) -> usize {
        self.width() as usize * self.bytes_per_pixel()
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn row(&self, row: u32) -> Option<&[u8]> {
        todo!("PixelBuffer::row")
    }

    pub fn row_mut(&mut self, row: u32) -> Option<&mut [u8]> {
        todo!("PixelBuffer::row_mut")
    }

    pub fn rows<'a>(&'a self) -> impl ExactSizeIterator + DoubleEndedIterator<Item = &'a [u8]> {
        self.pixels.chunks(self.row_len())
    }

    pub fn rows_mut<'a>(
        &'a mut self,
    ) -> impl ExactSizeIterator + DoubleEndedIterator<Item = &'a mut [u8]> {
        let chunk_size = self.row_len();
        self.pixels.chunks_mut(chunk_size)
    }
}
