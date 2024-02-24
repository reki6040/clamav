use crate::errors::{ErrorKind, Result};
use crate::one::property::charset::Charset;
use crate::one::property::color_ref::ColorRef;
use crate::one::property::paragraph_alignment::ParagraphAlignment;
use crate::one::property::{simple, PropertyType};
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;

/// A paragraph style.
///
/// See [\[MS-ONE\] 2.2.43] and [\[MS-ONE\] 2.2.44]
///
/// [\[MS-ONE\] 2.2.43]: https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-one/38eb9b74-cfaf-4df7-b061-a83968c7ff5b
/// [\[MS-ONE\] 2.2.44]: https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-one/f0baabae-f42a-42e0-8cb2-869d420e865f
#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) charset: Option<Charset>,
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) underline: bool,
    pub(crate) strikethrough: bool,
    pub(crate) superscript: bool,
    pub(crate) subscript: bool,
    pub(crate) font: Option<String>,
    pub(crate) font_size: Option<u16>,
    pub(crate) font_color: Option<ColorRef>,
    pub(crate) highlight: Option<ColorRef>,
    pub(crate) next_style: Option<String>,
    pub(crate) style_id: Option<String>,
    pub(crate) paragraph_alignment: Option<ParagraphAlignment>,
    pub(crate) paragraph_space_before: Option<f32>,
    pub(crate) paragraph_space_after: Option<f32>,
    pub(crate) paragraph_line_spacing_exact: Option<f32>,
    pub(crate) language_code: Option<u32>,
    pub(crate) math_formatting: bool,
    pub(crate) hyperlink: bool,
    pub(crate) hyperlink_protected: bool,
    pub(crate) hidden: bool,
    pub(crate) text_run_is_embedded_object: bool,
    pub(crate) text_run_object_type: Option<u32>,
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::ParagraphStyleObject.as_jcid() {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let charset = Charset::parse(PropertyType::Charset, object)?;
    let bold = simple::parse_bool(PropertyType::Bold, object)?.unwrap_or_default();
    let italic = simple::parse_bool(PropertyType::Italic, object)?.unwrap_or_default();
    let underline = simple::parse_bool(PropertyType::Underline, object)?.unwrap_or_default();
    let strikethrough =
        simple::parse_bool(PropertyType::Strikethrough, object)?.unwrap_or_default();
    let superscript = simple::parse_bool(PropertyType::Superscript, object)?.unwrap_or_default();
    let subscript = simple::parse_bool(PropertyType::Subscript, object)?.unwrap_or_default();
    let font = simple::parse_string(PropertyType::Font, object)?;
    let font_size = simple::parse_u16(PropertyType::FontSize, object)?;
    let font_color = ColorRef::parse(PropertyType::FontColor, object)?;
    let highlight = ColorRef::parse(PropertyType::Highlight, object)?;
    let next_style = simple::parse_string(PropertyType::NextStyle, object)?;
    let style_id = simple::parse_string(PropertyType::ParagraphStyleId, object)?;
    let paragraph_alignment = ParagraphAlignment::parse(object)?;
    let paragraph_space_before = simple::parse_f32(PropertyType::ParagraphSpaceBefore, object)?;
    let paragraph_space_after = simple::parse_f32(PropertyType::ParagraphSpaceAfter, object)?;
    let paragraph_line_spacing_exact =
        simple::parse_f32(PropertyType::ParagraphLineSpacingExact, object)?;
    let language_code = simple::parse_u32(PropertyType::LanguageId, object)?;
    let math_formatting =
        simple::parse_bool(PropertyType::MathFormatting, object)?.unwrap_or_default();
    let hyperlink = simple::parse_bool(PropertyType::Hyperlink, object)?.unwrap_or_default();
    let hyperlink_protected =
        simple::parse_bool(PropertyType::HyperlinkProtected, object)?.unwrap_or_default();
    let hidden = simple::parse_bool(PropertyType::Hidden, object)?.unwrap_or_default();
    let text_run_is_embedded_object =
        simple::parse_bool(PropertyType::TextRunIsEmbeddedObject, object)?.unwrap_or_default();
    let text_run_object_type = simple::parse_u32(PropertyType::EmbeddedObjectType, object)?;

    let data = Data {
        charset,
        bold,
        italic,
        underline,
        strikethrough,
        superscript,
        subscript,
        font,
        font_size,
        font_color,
        highlight,
        next_style,
        style_id,
        paragraph_alignment,
        paragraph_space_before,
        paragraph_space_after,
        paragraph_line_spacing_exact,
        language_code,
        math_formatting,
        hyperlink,
        hyperlink_protected,
        hidden,
        text_run_is_embedded_object,
        text_run_object_type,
    };

    Ok(data)
}
