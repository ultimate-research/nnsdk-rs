use alloc::{string::String, string::ToString, vec, vec::Vec};
use core::convert::TryFrom;
use binrw::io::{Cursor, Read, Seek, SeekFrom, TakeSeekExt};
use binrw::meta::{EndianKind, ReadEndian};
use binrw::{binread, BinRead, BinResult, NullString, Endian, BinWrite, BinWriterExt, binwrite};
use byteorder::{LittleEndian, ReadBytesExt}; // 1.2.7
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};

#[allow(unused_imports)]
use self::super::root;

use core::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug, Copy, Clone)]
pub struct ResColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug, Copy, Clone)]
pub struct ResVec2 {
    pub x: f32,
    pub y: f32,
}

impl ReadEndian for ResVec2 {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl ResVec2 {
    pub fn default() -> ResVec2 {
        ResVec2 { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> ResVec2 {
        ResVec2 { x, y }
    }
}


#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug, Copy, Clone)]
pub struct ResVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ReadEndian for ResVec3 {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl ResVec3 {
    pub fn default() -> ResVec3 {
        ResVec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> ResVec3 {
        ResVec3 { x, y, z }
    }
}

#[derive(Debug, BinRead, BinWrite, Default, Clone)]
pub struct SerdeNullString(NullString);

impl Serialize for SerdeNullString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SerdeNullString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        deserializer.deserialize_str(SerdeNullStringVisitor)
    }
}

struct SerdeNullStringVisitor;

impl<'de> Visitor<'de> for SerdeNullStringVisitor {
    type Value = SerdeNullString;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "a string to be converted to null string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where E: de::Error
    {
        let null_str = NullString::from(s);

        Ok(SerdeNullString(null_str))
    }
}

pub unsafe fn str_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> &str {
    let nul_range_end = utf8_src.iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    ::core::str::from_utf8_unchecked(&utf8_src[0..nul_range_end])
}

fn cstr_serialize<S>(x: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(unsafe {
        str_from_u8_nul_utf8_unchecked(x)
    })
}

struct CstrVisitor;

impl<'de> Visitor<'de> for CstrVisitor {
    type Value = [u8; 24];

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "a string to be converted to byte array")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where E: de::Error
    {
        let mut bytes = [0u8; 24];
        let null_str = s.as_bytes();
        assert!(null_str.len() <= 24);
        for (idx, byte) in null_str.bytes().enumerate() {
            bytes[idx] = byte.unwrap();    
        }

        Ok(bytes)
    }
}

fn cstr_deserialize<'de, D>(d: D) -> Result<[u8; 24], D::Error>
where
    D: Deserializer<'de>,
{
    let buf = d.deserialize_str(CstrVisitor)?;
    Ok(buf)
}

impl ReadEndian for SerdeNullString {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}


#[derive(Serialize, Deserialize, Debug)]
#[binread]
#[binwrite]
#[brw(little, magic = b"FLYT")]
pub struct BflytFile {
    header: BflytHeader,
    #[br(count = header.section_count)]
    sections: Vec<BflytSection>,
}

#[binread]
#[binwrite]
#[derive(Serialize, Deserialize, Debug)]
pub struct BflytHeader {
    byte_order: u16,
    header_size: u16,
    version: u32,
    file_size: u32,
    section_count: u16,
    padding: u16
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug, Clone)]
pub struct ResPane {
    pub flag: u8,
    pub base_position: u8,
    pub alpha: u8,
    pub flag_ex: u8,
    #[serde(serialize_with = "cstr_serialize", deserialize_with = "cstr_deserialize")]
    pub name: [u8; 24],
    pub user_data: [u8; 8],
    pub pos: ResVec3,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

impl ReadEndian for ResPane {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

fn texture_list_parser<R: Read + Seek>(reader: &mut R, _: Endian, _: ()) -> BinResult<TextureListInner> {
    let mut texture_names: Vec<SerdeNullString> = Vec::new();

    let tex_count = reader.read_i32::<LittleEndian>()?;
    let base_offset = reader.stream_position()?;

    let mut offsets = vec![0i32; tex_count as usize];
    reader.read_i32_into::<LittleEndian>(offsets.as_mut_slice())?;
    for offset in &offsets {
        reader.seek(SeekFrom::Start(base_offset + *offset as u64))?;
        texture_names.push(SerdeNullString::read(reader)?);
    }

    Ok(TextureListInner { tex_count, offsets, texture_names })
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct TextureListInner {
    pub tex_count: i32,
    #[br(count = tex_count)]
    pub offsets: Vec<i32>,
    #[br(count = tex_count)]
    pub texture_names: Vec<SerdeNullString>
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug, Clone)]
pub struct ResPicture {
    pub pane: ResPane,
    pub vtx_cols: [ResColor; 4],
    pub material_idx: u16,
    pub tex_coord_count: u8,
    pub flags: u8,
    #[br(count = tex_coord_count)]
    pub tex_coords: Vec<[ResVec2; 4]>,
}

#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug, Default)]
pub struct ResAnimationInfo {
    pub kind: u32,
    pub count: u8,
    pub padding: [u8; 3],
}

impl ReadEndian for ResAnimationInfo {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct ResPerCharacterTransform {
    pub eval_time_offset: f32,
    pub eval_time_width: f32,
    pub loop_type: u8,
    pub origin_v: u8,
    pub has_animation_info: u8,
    pub padding: [u8; 1],
}

impl ReadEndian for ResPerCharacterTransform {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct ResTextBox {
    pub pane: ResPane,
    pub text_buf_bytes: u16,
    pub text_str_bytes: u16,
    pub material_idx: u16,
    pub font_idx: u16,
    pub text_position: u8,
    pub text_alignment: u8,
    pub text_box_flag: u16,
    pub italic_ratio: f32,
    pub text_str_offset: u32,
    pub text_cols: [ResColor; 2],
    pub font_size: ResVec2,
    pub char_space: f32,
    pub line_space: f32,
    pub text_id_offset: u32,
    pub shadow_offset: ResVec2,
    pub shadow_scale: ResVec2,
    pub shadow_cols: [ResColor; 2],
    pub shadow_italic_ratio: f32,
    pub line_width_offset_offset: u32,
    pub per_character_transform_offset: u32,
    #[br(count = text_buf_bytes)]
    pub text: Vec<u8>,
    #[bw(
        if(*text_id_offset > 0), 
        // Can't use absolute offsets, so... we know it's after the text string.
        pad_before = (*text_id_offset as u64 - (*text_str_offset as u64 + *text_buf_bytes as u64))
    )]
    #[br(
        if(text_id_offset > 0), 
        // Can't use absolute offsets, so... we know it's after the text string.
        pad_before = (text_id_offset as u64 - (text_str_offset as u64 + text_buf_bytes as u64))
    )]
    pub text_id: SerdeNullString,
    // Not sure if any of the following work, so if something breaks, check here.
    #[br(if(line_width_offset_offset > 0))]
    #[bw(if(*line_width_offset_offset > 0))]
    pub line_width_offset_count: u8,
    #[br(if(line_width_offset_offset > 0), count = line_width_offset_count)]
    #[bw(if(*line_width_offset_offset > 0))]
    pub line_offset: Vec<f32>,
    #[br(if(line_width_offset_offset > 0), count = line_width_offset_count)]
    #[bw(if(*line_width_offset_offset > 0))]
    pub line_width: Vec<f32>,
    #[br(if(per_character_transform_offset > 0))]
    #[bw(if(*per_character_transform_offset > 0))]
    pub per_character_transform: Option<ResPerCharacterTransform>,
    #[br(if(per_character_transform_offset > 0))]
    #[bw(if(*per_character_transform_offset > 0))]
    pub per_character_transform_animation_info: Option<ResAnimationInfo>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct ResFont {
    offset: u32
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct FontListInner {
    pub font_count: u16,
    padding: u16,
    #[br(count = font_count)]
    pub fonts: Vec<ResFont>,
    #[br(count = font_count)]
    pub font_names: Vec<SerdeNullString>
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct ResPartsProperty {
    #[serde(serialize_with = "cstr_serialize", deserialize_with = "cstr_deserialize")]
    pub name: [u8; 24],
    pub usage_flag: u8,
    pub basic_usage_flag: u8,
    pub material_usage_flag: u8,
    pub system_ext_user_data_override_flag: u8,
    pub property_offset: u32,
    pub ext_user_data_offset: u32,
    pub pane_basic_info_offset: u32,
}

impl ReadEndian for ResPartsProperty {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct ResParts {
    pub size: u32,
    pub pane: ResPane,
    pub property_count: u32,
    pub magnify: ResVec2,
    #[br(count = property_count)]
    pub properties: Vec<ResPartsProperty>,
    #[brw(align_after = 4)]
    pub part_name: SerdeNullString,
    // Not actually
    #[br(count = property_count)]
    pub sections: Vec<BflytSection>
}

#[repr(C)]
#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub struct ResPartsPaneBasicInfo {
    pub user_data: [u8; 8],
    pub translate: ResVec3,
    pub rotate: ResVec3,
    pub scale: ResVec2,
    pub size: ResVec2,
    pub alpha: u8,
    padding: [u8; 3]
}

fn res_parts_parser<R: Read + Seek>(reader: &mut R, _: Endian, _: ()) -> BinResult<ResParts> {
    let base_offset = reader.stream_position()? - 4;

    let size = reader.read_u32::<LittleEndian>()?;
    let pane = ResPane::read(reader)?;

    let mut properties: Vec<ResPartsProperty> = Vec::new();

    let property_count = reader.read_u32::<LittleEndian>()?;
    let magnify = ResVec2::read(reader)?;

    for _ in 0..property_count {
        let property = ResPartsProperty::read(reader)?;
        properties.push(property);
    }

    let part_name = SerdeNullString::read(reader)?;
    let pos = reader.stream_position()?;
    if pos % 4 != 0 {
        reader.seek(SeekFrom::Current((4 - (pos % 4)) as i64))?;
    }

    let mut sections = Vec::new();
    for property in &properties {
        if property.property_offset != 0 {
            reader.seek(SeekFrom::Start(base_offset + property.property_offset as u64))?;
            let section = BflytSection::read(reader)?;
            sections.push(section);
        }

        if property.ext_user_data_offset != 0 {
            reader.seek(SeekFrom::Start(base_offset + property.ext_user_data_offset as u64))?;
            let section = BflytSection::read(reader)?;
            sections.push(section);
        }

        if property.pane_basic_info_offset != 0 {
            reader.seek(SeekFrom::Start(base_offset + property.pane_basic_info_offset as u64))?;
            let section = BflytSection::read(reader)?;
            sections.push(section);
        }
    }

    let curr_pos = reader.stream_position()?;
    assert!(curr_pos == base_offset + size as u64, "Failed to parse ResParts with pane name {} and part name {:#?}. Expected to read {size} bytes, but read {}", 
        unsafe { str_from_u8_nul_utf8_unchecked(&pane.name) }, part_name, curr_pos - base_offset);

    let parts = ResParts {
        size,
        pane,
        property_count,
        magnify,
        properties,
        part_name,
        sections
    };

    Ok(parts)
}

#[derive(Serialize, Deserialize, BinRead, BinWrite, Debug)]
pub enum BflytSection {
    #[brw(magic = b"pan1")]
    Pane {
        size: u32,
        pane: ResPane
    },

    #[brw(magic = b"txl1")]
    TextureList {
        size: u32,
        #[br(parse_with = texture_list_parser)]
        #[brw(align_after = 4)]
        texture_list: TextureListInner
    },

    #[brw(magic = b"pic1")]
    Picture {
        size: u32,
        picture: ResPicture
    },

    #[brw(magic = b"txt1")]
    TextBox {
        size: u32,
        #[brw(align_after = 4)]
        text_box: ResTextBox
    },

    #[brw(magic = b"prt1")]
    Part {
        #[br(parse_with = res_parts_parser)]
        part: ResParts,
    },

    #[brw(magic = b"mat1")]
    Material {
        size: u32,
        #[br(count = size as usize - 8)]
        data: Vec<u8>,
    },

    #[brw(magic = b"wnd1")]
    Window {
        size: u32,
        #[br(count = size as usize - 8)]
        data: Vec<u8>,
    },

    #[brw(magic = b"pas1")]
    PaneStart {
        #[br(assert(size == 8))]
        size: u32,
    },

    #[brw(magic = b"pae1")]
    PaneEnd {
        #[br(assert(size == 8))]
        size: u32,
    },

    #[brw(magic = b"grp1")]
    Group {
        size: u32,
        #[br(count = size as usize - 8)]
        data: Vec<u8>,
    },

    #[brw(magic = b"grs1")]
    GroupStart {
        #[br(assert(size == 8))]
        size: u32,
    },

    #[brw(magic = b"gre1")]
    GroupEnd {
        #[br(assert(size == 8))]
        size: u32,
    },

    #[brw(magic = b"bnd1")]
    Bounding {
        size: u32,
        #[br(count = size as usize - 8)]
        data: Vec<u8>,
    },

    #[brw(magic = b"lyt1")]
    Layout {
        size: u32,
        #[br(count = size as usize - 8)]
        data: Vec<u8>,
    },

    #[brw(magic = b"fnl1")]
    FontList {
        size: u32,
        #[brw(align_after = 4)]
        font_list: FontListInner
    },

    #[brw(magic = b"usd1")]
    UserDataList {
        size: u32,
        #[br(count = size as usize - 8)]
        data: Vec<u8>,
    },

    PartsBasicInfo {
        info: ResPartsPaneBasicInfo
    }
}

impl ReadEndian for BflytSection {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}
