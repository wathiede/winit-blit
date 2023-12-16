use std::io;

use log::{debug, error};
use raw_window_handle::{RawWindowHandle, WebWindowHandle};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::{PixelBufferCreationError, PixelBufferFormatSupported, PixelBufferFormatType};

pub struct PixelBuffer {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    data: Clamped<Vec<u8>>,
    width: u32,
    height: u32,
}

impl PixelBuffer {
    pub unsafe fn new(
        width: u32,
        height: u32,
        format: PixelBufferFormatType,
        raw_window_handle: RawWindowHandle,
    ) -> Result<PixelBuffer, PixelBufferCreationError> {
        debug!(
            "wasm32 PixelBuffer::new {} {} {:?} {:?}",
            width, height, format, raw_window_handle
        );

        let raw_handle_id =
            if let RawWindowHandle::Web(WebWindowHandle { id, .. }) = raw_window_handle {
                id
            } else {
                return Err(PixelBufferCreationError::FormatNotSupported);
            };

        let window = web_sys::window().ok_or_else(|| {
            error!("failed to find window");
            PixelBufferCreationError::FormatNotSupported
        })?;
        let document = window.document().ok_or_else(|| {
            error!("failed to find document");
            PixelBufferCreationError::FormatNotSupported
        })?;
        // Now find the canvas with this raw handle id.
        let canvases = document.get_elements_by_tag_name("canvas");
        let mut canvas = None;
        for idx in 0..canvases.length() {
            let c = canvases
                .item(idx)
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| {
                    error!("Couldn't cast canvas {} to HtmlCanvasElement", idx);
                    PixelBufferCreationError::FormatNotSupported
                })?;
            // "raw-handle" is from the `raw_window_handle::web::WebWindowHandle` documentation for
            // `id()`.
            // However, javascript access is camelCased according to
            // https://developer.mozilla.org/en-US/docs/Web/API/HTMLOrForeignElement/dataset
            if let Some(id) = c.dataset().get("rawHandle") {
                if raw_handle_id
                    == id
                        .parse()
                        // raw_window_handle should never be 0 for a valid canvas according to
                        // https://docs.rs/raw-window-handle/0.6/raw_window_handle/web/struct.WebWindowHandle.html
                        .unwrap_or(0)
                {
                    canvas = Some(c);
                    break;
                }
            }
        }
        let canvas = canvas.ok_or_else(|| {
            error!(
                "failed to find canvas matching raw handle id {}",
                raw_handle_id
            );
            PixelBufferCreationError::FormatNotSupported
        })?;
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let data = Clamped(vec![0; width as usize * height as usize * 4]);
        Ok(PixelBuffer {
            canvas,
            ctx,
            data,
            width,
            height,
        })
    }

    pub unsafe fn blit(&self, handle: RawWindowHandle) -> io::Result<()> {
        debug!("wasm32 PixelBuffer::blit {:?}", handle);
        let imagedata = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.data.0),
            self.width,
            self.height,
        )
        .map_err(|e| {
            error!("failed to create image data {:?}", e);
            io::Error::new(io::ErrorKind::InvalidData, "failed to create image data")
        })?;
        self.ctx.put_image_data(&imagedata, 0., 0.).map_err(|e| {
            error!("failed to put image data {:?}", e);
            io::Error::new(io::ErrorKind::InvalidData, "failed to put image data")
        })?;
        Ok(())
    }

    pub unsafe fn blit_rect(
        &self,
        src_pos: (u32, u32),
        dst_pos: (u32, u32),
        blit_size: (u32, u32),
        handle: RawWindowHandle,
    ) -> io::Result<()> {
        todo!(
            "wasm32 PixelBuffer::blit_rect {:?} {:?} {:?} {:?}",
            src_pos,
            dst_pos,
            blit_size,
            handle
        );
    }
    pub fn bits_per_pixel(&self) -> usize {
        32
    }
    pub fn bytes_per_pixel(&self) -> usize {
        self.bits_per_pixel() / 8
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
        todo!("wasm32 PixelBuffer::row {}", row)
    }
    pub fn row_mut(&mut self, row: u32) -> Option<&mut [u8]> {
        todo!("wasm32 PixelBuffer::row_mut {}", row)
    }
    pub fn rows<'a>(&'a self) -> impl ExactSizeIterator + DoubleEndedIterator<Item = &'a [u8]> {
        self.data.chunks(self.row_len())
    }
    pub fn rows_mut<'a>(
        &'a mut self,
    ) -> impl ExactSizeIterator + DoubleEndedIterator<Item = &'a mut [u8]> {
        let row_len = self.row_len();
        self.data.chunks_mut(row_len)
    }
}

impl PixelBufferFormatSupported for crate::RGBA {}
pub type NativeFormat = crate::RGBA;
