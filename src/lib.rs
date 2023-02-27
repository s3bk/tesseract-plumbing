mod tess_base_api;
mod text;
mod result_iterator;

use self::tesseract_sys::TessVersion;
pub use leptonica_plumbing;
pub use leptonica_plumbing::leptonica_sys;
use std::ffi::CStr;
pub use tess_base_api::{
    TessBaseApi, TessBaseApiGetAltoTextError, TessBaseApiGetHocrTextError,
    TessBaseApiGetLstmBoxTextError, TessBaseApiGetTsvTextError, TessBaseApiGetUtf8TextError,
    TessBaseApiGetWordStrBoxTextError, TessBaseApiInitError, TessBaseApiRecogniseError,
    TessBaseApiSetImageSafetyError, TessBaseApiSetVariableError, PageSegMode,
};
pub use tesseract_sys;
pub use text::Text;
pub use result_iterator::{Bbox, ResultIterator, FontAttributes};

pub enum Level {
    Block,    // Block of text/image/separator line.
    Para,     // Paragraph within a block.
    Textline, // Line within a paragraph.
    Word,     // Word within a textline.
    Symbol    // Symbol/character within a word.

/// Wrapper for [`Version`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a3785779c909fcdd77e24b340f5913e4b)
///
/// Returns the version identifier as a static string.
pub fn version() -> &'static CStr {
    unsafe { CStr::from_ptr(TessVersion()) }
}

pub use result_iterator::{Bbox, ResultIterator, FontAttributes};

pub enum Level {
    Block,    // Block of text/image/separator line.
    Para,     // Paragraph within a block.
    Textline, // Line within a paragraph.
    Word,     // Word within a textline.
    Symbol    // Symbol/character within a word.
}
