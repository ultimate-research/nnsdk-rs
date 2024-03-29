use alloc::string::String;

#[allow(unused_imports)]
use self::super::root;

use core::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResVec2 {
    pub x: f32,
    pub y: f32,
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
#[derive(Debug, Copy, Clone)]
pub struct ResVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

// Maybe needs a vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPane {
    pub block_header_kind: u32,
    pub block_header_size: u32,
    pub flag: u8,
    pub base_position: u8,
    pub alpha: u8,
    pub flag_ex: u8,
    pub name: [libc::c_char; 24],
    pub user_data: [libc::c_char; 8],
    pub pos: ResVec3,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

impl ResPane {
    // For null pane
    pub fn new(name: &str) -> ResPane {
        let mut pane = ResPane {
            block_header_kind: u32::from_le_bytes([b'p', b'a', b'n', b'1']),
            block_header_size: 84,
            /// Visible | InfluencedAlpha
            flag: 0x3,
            base_position: 0,
            alpha: 0xFF,
            flag_ex: 0,
            name: [0; 24],
            user_data: [0; 8],
            pos: ResVec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            size_x: 30.0,
            size_y: 40.0,
        };
        pane.set_name(name);
        pane
    }

    pub fn set_name(&mut self, name: &str) {
        assert!(
            name.len() <= 24,
            "Name of pane must be at most 24 characters"
        );
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), self.name.as_mut_ptr(), name.len());
        }
    }

    pub fn set_pos(&mut self, pos: ResVec3) {
        self.pos = pos;
    }

    pub fn set_size(&mut self, size: ResVec2) {
        self.size_x = size.x;
        self.size_y = size.y;
    }

    pub fn get_name(&self) -> String {
        self.name
            .iter()
            .take_while(|b| **b != 0)
            .map(|b| *b as char)
            .collect::<String>()
    }

    pub fn name_matches(&self, other: &str) -> bool {
        self.get_name() == other
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
enum TextBoxFlag {
    ShadowEnabled,
    ForceAssignTextLength,
    InvisibleBorderEnabled,
    DoubleDrawnBorderEnabled,
    PerCharacterTransformEnabled,
    CenterCeilingEnabled,
    LineWidthOffsetEnabled,
    ExtendedTagEnabled,
    PerCharacterTransformSplitByCharWidth,
    PerCharacterTransformAutoShadowAlpha,
    DrawFromRightToLeft,
    PerCharacterTransformOriginToCenter,
    KeepingFontScaleEnabled,
    PerCharacterTransformFixSpace,
    PerCharacterTransformSplitByCharWidthInsertSpaceEnabled,
    Max,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TextAlignment {
    Synchronous,
    Left,
    Center,
    Right,
    MaxTextAlignment,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    /* Additional Info
        uint16_t           text[];                     // Text.
        char                textId[];                   // The text ID.
        u8 lineWidthOffsetCount; // The quantity of widths and offsets for each line.
        float lineOffset[]; // The offset for each line.
        float lineWidth[]; // The width of each line.
        ResPerCharacterTransform perCharacterTransform     // Information for per-character animation.
        ResAnimationInfo       perCharacterTransformAnimationInfo;     // Animation information for per-character animation.
    */
}

impl Deref for ResTextBox {
    type Target = ResPane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for ResTextBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

impl ResTextBox {
    pub fn enable_shadow(&mut self) {
        self.text_box_flag |= 0x1 << TextBoxFlag::ShadowEnabled as u8;
    }

    pub fn text_alignment(&mut self, align: TextAlignment) {
        self.text_alignment = align as u8;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPicture {
    pub pane: ResPane,
    pub vtx_cols: [ResColor; 4],
    pub material_idx: u16,
    pub tex_coord_count: u8,
    pub flags: u8,
    /* Additional Info
        ResVec2 texCoords[texCoordCount][VERTEX_MAX];
        uint32_t shapeBinaryIndex;
    */
}

impl Deref for ResPicture {
    type Target = ResPane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for ResPicture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPictureWithTex<const TEX_COORD_COUNT: usize> {
    pub picture: ResPicture,
    pub tex_coords: [[ResVec2; TEX_COORD_COUNT]; 4],
}

impl<const TEX_COORD_COUNT: usize> Deref for ResPictureWithTex<TEX_COORD_COUNT> {
    type Target = ResPane;

    fn deref(&self) -> &Self::Target {
        &self.picture
    }
}

impl<const TEX_COORD_COUNT: usize> DerefMut for ResPictureWithTex<TEX_COORD_COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.picture
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResParts {
    pub pane: ResPane,
    pub property_count: u32,
    pub magnify: ResVec2,
}

impl Deref for ResParts {
    type Target = ResPane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for ResParts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPartsProperty {
    pub name: [libc::c_char; 24],
    pub usage_flag: u8,
    pub basic_usage_flag: u8,
    pub material_usage_flag: u8,
    pub system_ext_user_data_override_flag: u8,
    pub property_offset: u32,
    pub ext_user_data_offset: u32,
    pub pane_basic_info_offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResPartsWithProperty<const PROPERTY_COUNT: usize> {
    pub parts: ResParts,
    pub property_table: [ResPartsProperty; PROPERTY_COUNT],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindowInflation {
    pub left: i16,
    pub right: i16,
    pub top: i16,
    pub bottom: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindowFrameSize {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindowContent {
    pub vtx_cols: [ResColor; 4],
    pub material_idx: u16,
    pub tex_coord_count: u8,
    pub padding: [u8; 1],
    /* Additional Info
        nn::util::Float2 texCoords[texCoordCount][VERTEX_MAX];
    */
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindowContentWithTexCoords<const TEX_COORD_COUNT: usize> {
    pub window_content: ResWindowContent,
    // This has to be wrong.
    // Should be [[ResVec2; TEX_COORD_COUNT]; 4]?
    pub tex_coords: [[ResVec3; TEX_COORD_COUNT]; 1],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindowFrame {
    pub material_idx: u16,
    pub texture_flip: u8,
    pub padding: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindow {
    pub pane: ResPane,
    pub inflation: ResWindowInflation,
    pub frame_size: ResWindowFrameSize,
    pub frame_count: u8,
    pub window_flags: u8,
    pub padding: [u8; 2],
    pub content_offset: u32,
    pub frame_offset_table_offset: u32,
    pub content: ResWindowContent,
    /* Additional Info

        ResWindowContent content;

        detail::uint32_t frameOffsetTable[frameCount];
        ResWindowFrame frames;

    */
}

impl Deref for ResWindow {
    type Target = ResPane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for ResWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResWindowWithTexCoordsAndFrames<const TEX_COORD_COUNT: usize, const FRAME_COUNT: usize> {
    pub window: ResWindow,
    pub content: ResWindowContentWithTexCoords<TEX_COORD_COUNT>,
    pub frame_offset_table: [u32; FRAME_COUNT],
    pub frames: [ResWindowFrame; FRAME_COUNT],
}

impl<const TEX_COORD_COUNT: usize, const FRAME_COUNT: usize> Deref
    for ResWindowWithTexCoordsAndFrames<TEX_COORD_COUNT, FRAME_COUNT>
{
    type Target = ResPane;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl<const TEX_COORD_COUNT: usize, const FRAME_COUNT: usize> DerefMut
    for ResWindowWithTexCoordsAndFrames<TEX_COORD_COUNT, FRAME_COUNT>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}
