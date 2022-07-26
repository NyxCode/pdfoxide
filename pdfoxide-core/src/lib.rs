use js_sys::{Function, SharedArrayBuffer, Uint8Array, Uint8ClampedArray};
use pdfium_render::bitmap::{PdfBitmap, PdfBitmapFormat};
use pdfium_render::error::PdfiumError;
use pdfium_render::pages::PdfPageIndex;
use pdfium_render::pdfium::Pdfium;
use pdfium_render::prelude::{PdfBitmapConfig, PdfDocument};
use std::cell::RefCell;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::{console, Performance};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const BUFFER_SIZE: usize = 1000 * 1000 * 50;
// TODO: maybeUninit
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

fn buffer_addr() -> *mut u8 {
    unsafe { BUFFER.as_mut_ptr() }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen]
pub struct PdfOxide {
    pdfium: &'static Pdfium,
}

#[wasm_bindgen]
impl PdfOxide {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<PdfOxide, JsValue> {
        let bindings = Pdfium::bind_to_system_library().map_err(error_to_js)?;

        Ok(Self {
            pdfium: Box::leak(Box::new(Pdfium::new(bindings))),
        })
    }

    pub fn load_document(&self, data: &[u8]) -> Result<Document, JsValue> {
        let doc = self
            .pdfium
            .load_pdf_from_bytes(data, None)
            .map_err(error_to_js)?;
        Ok(Document { doc })
    }
}

#[wasm_bindgen]
pub struct Document {
    doc: PdfDocument<'static>,
}

#[wasm_bindgen]
impl Document {
    pub fn pages(&self) -> u16 {
        self.doc.pages().len()
    }

    pub fn render_page(&self, n: u16, width: u16, callback: Function) -> Result<(), JsValue> {
        console::time_with_label("get page from document");
        let pages = self.doc.pages();
        let page = pages.get(n).map_err(error_to_js)?;
        console::time_end_with_label("get page from document");

        console::time_with_label("get_bitmap_with_config");
        let bitmap_cfg = PdfBitmapConfig::new()
            .set_format(PdfBitmapFormat::BRGx)
            .set_target_width(width);
        let mut bitmap = page
            .get_bitmap_with_config(&bitmap_cfg)
            .map_err(error_to_js)?;
        console::time_end_with_label("get_bitmap_with_config");

        console::time_with_label("as_uint8array");
        let bytes = bitmap.as_uint8array();
        console::time_end_with_label("as_uint8array");

        console::time_with_label("as_uint8array");
        let clamped = Uint8ClampedArray::new_with_byte_offset_and_length(
            &bytes.buffer(),
            bytes.byte_offset(),
            bytes.byte_length(),
        );
        console::time_end_with_label("as_uint8array");

        console::time_with_label("callback into JS");
        callback.call3(
            &JsValue::NULL,
            &JsValue::from(bitmap.width()),
            &JsValue::from(bitmap.height()),
            &clamped,
        );
        console::time_end_with_label("callback into JS");

        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn error_to_js(err: PdfiumError) -> JsValue {
    JsValue::from_str(&format!("{:?}", err))
}

fn convert(a: &Uint8Array) -> Uint8ClampedArray {
    Uint8ClampedArray::new_with_byte_offset_and_length(a, a.byte_offset(), a.byte_length())
}
