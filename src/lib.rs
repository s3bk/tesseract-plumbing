mod tess_base_api;
mod text;
mod result_iterator;

pub use leptonica_plumbing;
pub use leptonica_plumbing::leptonica_sys;
pub use tess_base_api::{
    TessBaseApi, TessBaseApiGetAltoTextError, TessBaseApiGetHocrTextError,
    TessBaseApiGetLstmBoxTextError, TessBaseApiGetTsvTextError, TessBaseApiGetUtf8TextError,
    TessBaseApiGetWordStrBoxTextError, TessBaseApiInitError, TessBaseApiRecogniseError,
    TessBaseApiSetImageSafetyError, TessBaseApiSetVariableError,
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
}
