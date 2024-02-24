use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::one::property::object_reference::ObjectReference;
use crate::one::property::{simple, PropertyType};
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;
use crate::shared::multi_byte;

/// An ink stroke.
#[allow(dead_code)]
pub(crate) struct Data {
    pub(crate) path: Vec<i64>,
    pub(crate) bias: InkBias,
    pub(crate) language_code: Option<u32>,
    pub(crate) properties: ExGuid,
}

pub(crate) enum InkBias {
    Handwriting,
    Drawing,
    Both,
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::InkStrokeNode.as_jcid() {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let path = simple::parse_vec(PropertyType::InkPath, object)?
        .map(|data| multi_byte::decode_signed(&data))
        .ok_or_else(|| {
            ErrorKind::MalformedOneNoteFileData("ink stroke node has no ink path".into())
        })?;
    let bias = simple::parse_u8(PropertyType::InkBias, object)?
        .map(|bias| match bias {
            0 => Ok(InkBias::Handwriting),
            1 => Ok(InkBias::Drawing),
            2 => Ok(InkBias::Both),
            i => Err(ErrorKind::MalformedOneNoteFileData(
                format!("invalid ink bias value: {}", i).into(),
            )),
        })
        .transpose()?
        .ok_or_else(|| {
            ErrorKind::MalformedOneNoteFileData("ink stroke node has no ink bias".into())
        })?;
    let language_code = simple::parse_u32(PropertyType::LanguageId, object)?;
    let properties = ObjectReference::parse(PropertyType::InkStrokeProperties, object)?
        .ok_or_else(|| {
            ErrorKind::MalformedOneNoteFileData(
                "ink stroke node has no ink stroke properties".into(),
            )
        })?;

    let data = Data {
        path,
        bias,
        language_code,
        properties,
    };

    Ok(data)
}
