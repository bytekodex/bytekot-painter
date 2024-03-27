use std::ffi::{c_int, c_uchar};
#[allow(non_upper_case_globals)]
use std::ffi::CStr;
use std::io::Write;
use std::os::raw::c_char;
use std::ptr::null;

use antlr_rust::InputStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::token::TOKEN_EOF;
use antlr_rust::token_stream::{TokenStream, UnbufferedTokenStream};
use cached::proc_macro::cached;
use skia_safe::{ClipOp, Color, EncodedImageFormat, FontMgr, Point, Rect, RRect, scalar, surfaces, Vector};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};

use antlr::jbytecodelexer::{BytecodeAccFlag, BytecodeConstantPoolTag, BytecodeDescriptor, BytecodeInstr, BytecodeKeyword, BytecodeLiterals, BytecodeMethodReference, BytecodeNumber, BytecodePrimitive, BytecodeSignature, FilePathIdentifier, QualifiedIdentifier, SlCommentLiteral, SpecialPrimitives, StringLiteral};

use crate::antlr::jbytecodelexer::JBytecodeLexer;

mod antlr;

const FILEPATH_IDENTIFIER_COLOR: Color = Color::from_rgb(169, 169, 169);
const BYTECODE_PRIMITIVE_COLOR: Color = Color::from_rgb(86, 156, 214);
const BYTECODE_LITERALS_COLOR: Color = Color::from_rgb(86, 156, 214);
const SPECIAL_PRIMITIVES_COLOR: Color = Color::from_rgb(86, 156, 214);
const STRING_LITERAL_COLOR: Color = Color::from_rgb(214, 157, 133);
const QUALIFIED_IDENTIFIER_COLOR: Color = Color::from_rgb(78, 201, 176);
const BYTECODE_KEYWORD_COLOR: Color = Color::from_rgb(86, 156, 214);
const BYTECODE_CONSTANT_POOL_TAG_COLOR: Color = Color::from_rgb(156, 220, 254);
const BYTECODE_SIGNATURE_COLOR: Color = Color::from_rgb(214, 157, 133);
const BYTECODE_DESCRIPTOR_COLOR: Color = Color::from_rgb(214, 157, 133);
const BYTECODE_NUMBER_COLOR: Color = Color::from_rgb(181, 206, 168);
const BYTECODE_ACCFLAG_COLOR: Color = Color::from_rgb(187, 181, 41);
const BYTECODE_INSTR_COLOR: Color = Color::from_rgb(179, 137, 197);
const BYTECODE_METHODREFERENCE_COLOR: Color = Color::from_rgb(179, 137, 197);
const SL_COMMENT_LITERAL_COLOR: Color = Color::from_rgb(65, 165, 63);
const DEFAULT_COLOR: Color = Color::from_rgb(220, 220, 220);
const BACKGROUND_COLOR: Color = Color::from_rgb(30, 30, 30);

const FONTS: [&str; 1] = ["Fira code"];
const INITIAL_FONT_SIZE: f32 = 40.0;

const MAX_WIDTH: f32 = 3400.0;
const MAX_HEIGHT: f32 = 2560.0 * 3.2;

const CORNER_RADII: f32 = 20.0;
const MARGIN: i32 = 32;

const CORNERS: Point = Vector::new(CORNER_RADII, CORNER_RADII);

const ERR_SUCCESS: i32 = 0;
const ERR_TOO_LARGE_IMAGE: i32 = -1;
const ERR_RASTER_CREATION_FAILURE: i32 = -2;
const ERR_IMAGE_ENCODING_FAILURE: i32 = -3;

#[repr(C)]
pub struct ImageResult {
  data: *const c_uchar,
  len: usize,
  status: c_int,
}

#[no_mangle]
pub extern "C" fn paint(input: *const c_char) -> ImageResult {
  let input = unsafe { CStr::from_ptr(input).to_str().unwrap() };

  let mut paragraph_style = ParagraphStyle::new();
  let mut font_collection = FontCollection::new();
  let mut default_text_style = TextStyle::new();

  font_collection.set_default_font_manager(FontMgr::default(), FONTS[0]);

  default_text_style.set_font_families(&FONTS);
  default_text_style.set_color(DEFAULT_COLOR);
  default_text_style.set_font_size(INITIAL_FONT_SIZE);

  paragraph_style.set_text_style(&default_text_style);

  let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
  let mut lexer = JBytecodeLexer::new(InputStream::new(&*input));
  let mut token_source = UnbufferedTokenStream::new_unbuffered(&mut lexer);
  while token_source.la(1) != TOKEN_EOF {
    {
      let token = token_source.lt(1).unwrap();
      paragraph_builder.push_style(&style_from_token(categorize_token_type(token.token_type)));
      paragraph_builder.add_text(&token.text);
    }
    token_source.consume();
  }
  paragraph_builder.pop();

  let mut paragraph = paragraph_builder.build();
  paragraph.layout(0 as scalar); // Measure as small as we can.
  paragraph.layout(paragraph.max_intrinsic_width() as scalar); // So, we get some intrinsic width, use it.

  let width = paragraph.max_intrinsic_width();
  let height = paragraph.height();
  if height >= MAX_HEIGHT || (height * width > MAX_WIDTH * MAX_HEIGHT) {
    return ImageResult { data: null(), len: 0, status: ERR_TOO_LARGE_IMAGE };
  }

  let cm_width = width.ceil() as i32 + (MARGIN * 2);
  let cm_height = height.ceil() as i32 + (MARGIN * 2);
  let mut surface = match surfaces::raster_n32_premul((cm_width, cm_height)) {
    Some(surface) => surface,
    None => return ImageResult { data: null(), len: 0, status: ERR_RASTER_CREATION_FAILURE }
  };

  let canvas = surface.canvas();
  let rect = RRect::new_rect_radii(Rect::from_wh(cm_width as f32, cm_height as f32), &[CORNERS, CORNERS, CORNERS, CORNERS]);

  canvas.save();
  canvas.clip_rrect(&rect, ClipOp::Intersect, true);
  canvas.clear(BACKGROUND_COLOR);
  canvas.restore();

  paragraph.paint(canvas, (MARGIN, MARGIN));

  let snapshot = surface.image_snapshot();
  let image = match snapshot.encode(surface.direct_context().as_mut(), EncodedImageFormat::PNG, None) {
    Some(image) => image,
    None => return ImageResult { data: null(), len: 0, status: ERR_IMAGE_ENCODING_FAILURE }
  };

  let image_bytes = image.as_bytes().to_vec();
  let len = image_bytes.len();
  let ptr = image_bytes.as_ptr();
  std::mem::forget(image_bytes);

  return ImageResult { data: ptr, len, status: ERR_SUCCESS };
}

#[no_mangle]
pub extern "C" fn free_image_data(ptr: *mut c_uchar, len: usize) {
  unsafe {
    let _ = Vec::from_raw_parts(ptr, len, len);
  }
}

fn categorize_token_type(token: isize) -> isize {
  return if (17..=33).contains(&token) || (35..=37).contains(&token) {
    -1
  } else {
    token
  };
}

#[cached]
fn style_from_token(token: isize) -> TextStyle {
  let mut style = TextStyle::new();
  style.set_font_size(INITIAL_FONT_SIZE);

  match token {
    FilePathIdentifier => style.set_color(FILEPATH_IDENTIFIER_COLOR),
    BytecodePrimitive => style.set_color(BYTECODE_PRIMITIVE_COLOR),
    BytecodeLiterals => style.set_color(BYTECODE_LITERALS_COLOR),
    SpecialPrimitives => style.set_color(SPECIAL_PRIMITIVES_COLOR),
    StringLiteral => style.set_color(STRING_LITERAL_COLOR),
    QualifiedIdentifier => style.set_color(QUALIFIED_IDENTIFIER_COLOR),
    BytecodeKeyword => style.set_color(BYTECODE_KEYWORD_COLOR),
    BytecodeConstantPoolTag => style.set_color(BYTECODE_CONSTANT_POOL_TAG_COLOR),
    BytecodeSignature => style.set_color(BYTECODE_SIGNATURE_COLOR),
    BytecodeDescriptor => style.set_color(BYTECODE_DESCRIPTOR_COLOR),
    BytecodeNumber => style.set_color(BYTECODE_NUMBER_COLOR),
    BytecodeAccFlag => style.set_color(BYTECODE_ACCFLAG_COLOR),
    BytecodeInstr => style.set_color(BYTECODE_INSTR_COLOR),
    BytecodeMethodReference => style.set_color(BYTECODE_METHODREFERENCE_COLOR),
    SlCommentLiteral => style.set_color(SL_COMMENT_LITERAL_COLOR),
    _ => style.set_color(DEFAULT_COLOR),
  }.to_owned()
}