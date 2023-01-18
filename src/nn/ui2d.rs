#[allow(unused_imports)]
use self::super::root;

use core::ops::{Deref, DerefMut};

use alloc::string::String;

pub mod resources;
pub use resources::*;

#[repr(C)]
#[derive(Debug)]
pub struct ResAnimationContent {
    pub name: [libc::c_char; 28],
    pub count: u8,
    pub anim_content_type: u8,
    padding: [libc::c_char; 2],
}

/**
 * Block Header Kind
 *
 * ANIM_TAG: pat1
 * ANIM_SHARE: pah1
 * ANIM_INFO: pai1
 */

#[repr(C)]
#[derive(Debug)]
pub struct ResAnimationBlock {
    pub block_header_kind: u32,
    pub block_header_size: u32,
    pub num_frames: u16,
    pub is_loop: bool,
    pad: [libc::c_char; 1],
    pub file_count: u16,
    pub anim_cont_count: u16,
    pub anim_cont_offsets_offset: u32,
}

#[repr(C)]
pub struct AnimTransform {
    pub res_animation_block: *mut ResAnimationBlock,
    pub frame: f32,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct AnimTransformNode {
    pub prev: *mut AnimTransformNode,
    pub next: *mut AnimTransformNode,
}

#[repr(C)]
pub struct AnimTransformList {
    pub root: AnimTransformNode,
}

#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Pane {
    pub vtable: u64,
    pub link: PaneNode,
    pub parent: *mut Pane,
    pub children_list: PaneNode,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub size_x: f32,
    pub size_y: f32,
    pub flags: u8,
    pub alpha: u8,
    pub global_alpha: u8,
    pub base_position: u8,
    pub flag_ex: u8,
    // This is supposed to be 3 bytes padding + flags of 4 bytes + padding of 4 bytes
    pad: [u8; 3 + 4 + 4 + 8],
    pub global_matrix: [[f32; 3]; 4],
    pub user_matrix: *const u64,
    pub ext_user_data_list: *const u64,
    pub name: [libc::c_char; 25],
    pub user_data: [libc::c_char; 9],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PaneFlag {
    Visible,
    InfluencedAlpha,
    LocationAdjust,
    UserAllocated,
    IsGlobalMatrixDirty,
    UserMatrix,
    UserGlobalMatrix,
    IsConstantBufferReady,
    Max,
}

impl Pane {
    pub unsafe fn as_parts(&mut self) -> &mut Parts {
        &mut *(self as *mut Pane as *mut Parts)
    }

    pub unsafe fn as_picture(&mut self) -> &mut Picture {
        &mut *(self as *mut Pane as *mut Picture)
    }

    pub unsafe fn as_textbox(&mut self) -> &mut TextBox {
        &mut *(self as *mut Pane as *mut TextBox)
    }

    pub unsafe fn as_window(&mut self) -> &mut Window {
        &mut *(self as *mut Pane as *mut Window)
    }

    pub unsafe fn set_visible(&mut self, visible: bool) {
        if visible {
            self.alpha = 255;
            self.global_alpha = 255;
        } else {
            self.alpha = 0;
            self.global_alpha = 0;
        }
    }

    pub fn get_name(&self) -> String {
        self.name
            .iter()
            .take_while(|b| **b != 0)
            .map(|b| *b as char)
            .collect::<String>()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Parts {
    pub pane: Pane,
    // Some IntrusiveList
    pub link: PaneNode,
    pub layout: *mut Layout,
}

impl Deref for Parts {
    type Target = Pane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for Parts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Picture {
    pub pane: Pane,
    pub material: *mut Material,
    pub vertex_colors: [[u8; 4]; 4],
    pub shared_memory: *mut u8,
}

impl Deref for Picture {
    type Target = Pane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for Picture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TextBoxFlag {
    // TODO: Check if Horizontal is first or Vertical is
    TextAlignmentHorizontal,
    TextAlignmentVertical,
    IsPTDirty,
    ShadowEnabled,
    InvisibleBorderEnabled,
    DoubleDrawnBorderEnabled,
    WidthLimitEnabled,
    PerCharacterTransformEnabled,
    CenterCeilingEnabled,
    PerCharacterTransformSplitByCharWidth,
    PerCharacterTransformAutoShadowAlpha,
    DrawFromRightToLeft,
    PerCharacterTransformOriginToCenter,
    PerCharacterTransformFixSpace,
    LinefeedByCharacterHeightEnabled,
    PerCharacterTransformSplitByCharWidthInsertSpaceEnabled
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextBox {
    pub pane: Pane,
    // Actually a union
    pub m_text_buf: *mut libc::c_char,
    pub m_p_text_id: *const libc::c_char,
    pub m_text_colors: [[u8; 4]; 2],
    pub m_p_font: *const libc::c_void,
    pub m_font_size_x: f32,
    pub m_font_size_y: f32,
    pub m_line_space: f32,
    pub m_char_space: f32,

    // Actually a union
    pub m_p_tag_processor: *const libc::c_char,

    pub m_text_buf_len: u16,
    pub m_text_len: u16,

    // Use TextBoxFlag
    pub m_bits: u16,
    pub m_text_position: u8,

    pub m_is_utf8: bool,

    pub m_italic_ratio: f32,

    pub m_shadow_offset_x: f32,
    pub m_shadow_offset_y: f32,
    pub m_shadow_scale_x: f32,
    pub m_shadow_scale_y: f32,
    pub m_shadow_top_color: [u8; 4],
    pub m_shadow_bottom_color: [u8; 4],
    pub m_shadow_italic_ratio: f32,

    pub m_p_line_width_offset: *const libc::c_void,

    pub m_p_material: *mut Material,
    pub m_p_disp_string_buf: *const libc::c_void,

    pub m_p_per_character_transform: *const libc::c_void,
}

impl TextBox {
    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        let input_color = [r, g, b, a];
        let mut dirty: bool = false;
        self.m_text_colors
            .iter_mut()
            .for_each(|top_or_bottom_color| {
                if *top_or_bottom_color != input_color {
                    dirty = true;
                }
                *top_or_bottom_color = input_color;
            });

        if dirty {
            self.m_bits |= 1 << TextBoxFlag::IsPTDirty as u8;
        }
    }

    pub unsafe fn set_material_white_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        (*self.m_p_material).set_white_color(r, g, b, a);
    }

    pub unsafe fn set_material_black_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        (*self.m_p_material).set_black_color(r, g, b, a);
    }

    pub unsafe fn set_default_material_colors(&mut self) {
        self.set_material_white_color(255.0, 255.0, 255.0, 255.0);
        self.set_material_black_color(0.0, 0.0, 0.0, 255.0);
    }
}

impl Deref for TextBox {
    type Target = Pane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for TextBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[repr(C)]
pub union MaterialColor {
    pub byte_color: [[u8; 4]; 2],
    pub p_float_color: *mut *mut f32,
}

use core::fmt;
impl fmt::Debug for MaterialColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("MaterialColor")
                .field("byteColor", &self.byte_color)
                .field("pFloatColor", &self.p_float_color)
                .finish()
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum MaterialColorType {
    BlackColor,
    WhiteColor,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum MaterialFlags {
    UserAllocated,
    TextureOnly,
    ThresholdingAlphaInterpolation,
    BlackColorFloat,
    WhiteColorFloat,
    DynamicAllocatedColorData,
}

#[repr(C)]
#[derive(Debug)]
pub struct Material {
    pub vtable: u64,
    pub m_colors: MaterialColor,
    // Actually a struct
    m_mem_cap: u32,
    // Actually a struct
    m_mem_count: u32,
    pub m_p_mem: *mut libc::c_void,
    pub m_p_shader_info: *const libc::c_void,
    pub m_p_name: *const libc::c_char,
    pub m_vertex_shader_constant_buffer_offset: u32,
    pub m_pixel_shader_constant_buffer_offset: u32,
    pub m_p_user_shader_constant_buffer_information: *const libc::c_void,
    pub m_p_blend_state: *const libc::c_void,
    pub m_packed_values: u8,
    pub m_flag: u8,
    pub m_shader_variation: u16,
}

impl Material {
    pub fn set_color_int(&mut self, idx: usize, r: u8, g: u8, b: u8, a: u8) {
        let input_color = [r, g, b, a];
        unsafe {
            self.m_colors.byte_color[idx] = input_color;
        }
    }

    pub fn set_color_float(&mut self, idx: usize, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            *(*(self.m_colors.p_float_color.add(idx)).add(0)) = r;
            *(*(self.m_colors.p_float_color.add(idx)).add(1)) = g;
            *(*(self.m_colors.p_float_color.add(idx)).add(2)) = b;
            *(*(self.m_colors.p_float_color.add(idx)).add(3)) = a;
        }
    }

    pub fn set_color(&mut self, color_type: MaterialColorType, r: f32, g: f32, b: f32, a: f32) {
        let (is_float_flag, idx) = if color_type == MaterialColorType::BlackColor {
            (MaterialFlags::BlackColorFloat as u8, 0)
        } else {
            (MaterialFlags::WhiteColorFloat as u8, 1)
        };
        if self.m_flag & (0x1 << is_float_flag) != 0 {
            self.set_color_float(idx, r, g, b, a);
        } else {
            self.set_color_int(idx, r as u8, g as u8, b as u8, a as u8);
        }
    }

    pub fn set_white_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_color(MaterialColorType::WhiteColor, r, g, b, a);
    }

    pub fn set_black_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_color(MaterialColorType::BlackColor, r, g, b, a);
    }

    pub fn set_white_res_color(&mut self, white: ResColor) {
        self.set_white_color(white.r, white.g, white.b, white.a);
    }

    pub fn set_black_res_color(&mut self, black: ResColor) {
        self.set_black_color(black.r, black.g, black.b, black.a);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Window {
    pub pane: Pane,
    // TODO
}

impl Deref for Window {
    type Target = Pane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pane
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PaneNode {
    pub prev: *mut PaneNode,
    pub next: *mut PaneNode,
}

#[repr(C)]
pub struct Group {
    pub pane_list: PaneNode,
    pub name: *const libc::c_char,
}

#[repr(C)]
pub struct GroupContainer {}

#[repr(C)]
#[derive(Debug)]
pub struct Layout {
    pub vtable: u64,
    pub anim_trans_list: AnimTransformNode,
    pub root_pane: *mut Pane,
    pub group_container: u64,
    pub layout_size: f64,
    pub layout_name: *const libc::c_char,
}
