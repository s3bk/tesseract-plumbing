use std::ffi::CStr;
use std::marker::PhantomData;
use crate::{TessBaseApi, Text, Level};

pub struct ResultIterator<'a> {
    handle: *mut tesseract_sys::TessResultIterator,
    _m: PhantomData<&'a TessBaseApi>
}
impl<'a> ResultIterator<'a> {
    pub (crate) fn new(_: &'a TessBaseApi, handle: *mut tesseract_sys::TessResultIterator) -> Self {
        ResultIterator {
            handle,
            _m: PhantomData
        }
    }
    pub fn next(&mut self, level: Level) -> bool {
        unsafe {
            tesseract_sys::TessResultIteratorNext(self.handle, level as _) != 0
        }
    }
    pub fn is_at_beginning_of(&self, level: Level) -> bool {
        unsafe {
            tesseract_sys::TessPageIteratorIsAtBeginningOf(self.handle as _, level as _) != 0
        }
    }
    pub fn bounding_box(&self, level: Level) -> Option<Bbox> {
        let mut bbox = Bbox::default();
        unsafe {
            if tesseract_sys::TessPageIteratorBoundingBox(
                self.handle as *mut tesseract_sys::TessPageIterator,
                level as _,
                &mut bbox.left,
                &mut bbox.top,
                &mut bbox.right,
                &mut bbox.bottom
            ) != 0 {
                Some(bbox)
            } else {
                None
            }
        }
    }
    pub fn word_font_attributes(&self) -> Option<FontAttributes> {
        unsafe {
            let mut is_bold = 0;
            let mut is_italic = 0;
            let mut is_underlined = 0;
            let mut is_monospace = 0;
            let mut is_serif = 0;
            let mut is_smallcaps = 0;
            let mut pointsize = 0;
            let mut font_id = 0;
            let res = tesseract_sys::TessResultIteratorWordFontAttributes(
                self.handle,
                &mut is_bold,
                &mut is_italic,
                &mut is_underlined,
                &mut is_monospace,
                &mut is_serif,
                &mut is_smallcaps,
                &mut pointsize,
                &mut font_id
            );
            if res.is_null() {
                return None;
            }
            let font_name = CStr::from_ptr(res).to_str().unwrap();
            Some(FontAttributes {
                is_bold: is_bold != 0,
                is_italic: is_italic != 0,
                is_underlined: is_underlined != 0,
                is_monospace: is_monospace != 0,
                is_serif: is_serif != 0,
                is_smallcaps: is_smallcaps != 0,
                pointsize,
                font_id,
                font_name
            })
        }
    }
    pub fn confidence(&self, level: Level) -> f32 {
        unsafe {
            tesseract_sys::TessResultIteratorConfidence(self.handle, level as _)
        }
    }
    pub fn text(&self, level: Level) -> Option<Text> {
        unsafe {
            let ptr = tesseract_sys::TessResultIteratorGetUTF8Text(self.handle, level as _);
            if ptr.is_null() {
                return None;
            } else {
                Some(Text::new(ptr))
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Bbox {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug)]
pub struct FontAttributes<'a> {
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_underlined: bool,
    pub is_monospace: bool,
    pub is_serif: bool,
    pub is_smallcaps: bool,
    pub pointsize: i32,
    pub font_id: i32,
    pub font_name: &'a str
}
