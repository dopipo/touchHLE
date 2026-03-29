/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Implementation of OpenGL ES 1.1 on top of OpenGL 2.1 compatibility profile.
//!
//! The standard graphics drivers on most desktop operating systems do not
//! provide OpenGL ES 1.1, so we must provide it ourselves somehow.
//!
//! OpenGL ES 1.1 is based on OpenGL 1.5. Much of its core functionality (e.g.
//! the fixed-function pipeline) is considered legacy and not available in the
//! "core profile" for modern OpenGL versions, nor is it available at all in
//! later versions of OpenGL ES. However, OpenGL also has the "compatibility
//! profile" which still offers this legacy functionality.
//!
//! OpenGL 2.1 is the latest version that has a compatibility profile available
//! on macOS. It's also a version supported on various other OSes.
//! It is therefore a convenient target for our implementation.

use super::gl21compat_raw as gl21;
use super::gl21compat_raw::types::*;
use super::gles_generic::GLES;
use super::util::{
    fixed_to_float, matrix_fixed_to_float, try_decode_pvrtc, PalettedTextureFormat, ParamTable,
    ParamType,
};
use crate::window::{GLContext, GLVersion, Window};
use std::collections::HashSet;
use std::ffi::CStr;

/// List of capabilities shared by OpenGL ES 1.1 and OpenGL 2.1.
///
/// Note: There can be arbitrarily many lights or clip planes, depending on
/// implementation limits. We might eventually need to check those rather than
/// just providing the minimum.
pub const CAPABILITIES: &[GLenum] = &[
    gl21::ALPHA_TEST,
    gl21::BLEND,
    gl21::COLOR_LOGIC_OP,
    gl21::CLIP_PLANE0,
    gl21::CLIP_PLANE1,
    gl21::CLIP_PLANE2,
    gl21::CLIP_PLANE3,
    gl21::CLIP_PLANE4,
    gl21::CLIP_PLANE5,
    gl21::LIGHT0,
    gl21::LIGHT1,
    gl21::LIGHT2,
    gl21::LIGHT3,
    gl21::LIGHT4,
    gl21::LIGHT5,
    gl21::LIGHT6,
    gl21::LIGHT7,
    gl21::COLOR_MATERIAL,
    gl21::CULL_FACE,
    gl21::DEPTH_TEST,
    gl21::DITHER,
    gl21::FOG,
    gl21::LIGHTING,
    gl21::LINE_SMOOTH,
    gl21::MULTISAMPLE,
    gl21::NORMALIZE,
    gl21::POINT_SMOOTH,
    gl21::POLYGON_OFFSET_FILL,
    gl21::RESCALE_NORMAL,
    gl21::SAMPLE_ALPHA_TO_COVERAGE,
    gl21::SAMPLE_ALPHA_TO_ONE,
    gl21::SAMPLE_COVERAGE,
    gl21::SCISSOR_TEST,
    gl21::STENCIL_TEST,
    gl21::TEXTURE_2D,
    // Same as POINT_SPRITE_OES from the GLES extension
    gl21::POINT_SPRITE,
];

pub const UNSUPPORTED_CAPABILITIES: &[GLenum] = &[
    0x8620, // GL_VERTEX_PROGRAM_NV
    gl21::TEXTURE,
];

pub struct ArrayInfo {
    /// Enum used by `glEnableClientState`, `glDisableClientState` and
    /// `glGetBoolean`.
    pub name: GLenum,
    /// Buffer binding enum for `glGetInteger`.
    pub buffer_binding: GLenum,
    /// Size enum for `glGetInteger`.
    size: Option<GLenum>,
    /// Stride enum for `glGetInteger`.
    stride: GLenum,
    /// Pointer enum for `glGetPointer`.
    pub pointer: GLenum,
}

struct ArrayStateBackup {
    size: Option<GLint>,
    stride: GLsizei,
    pointer: *const GLvoid,
}

/// List of arrays shared by OpenGL ES 1.1 and OpenGL 2.1.
///
/// TODO: GL_POINT_SIZE_ARRAY_OES?
pub const ARRAYS: &[ArrayInfo] = &[
    ArrayInfo {
        name: gl21::COLOR_ARRAY,
        buffer_binding: gl21::COLOR_ARRAY_BUFFER_BINDING,
        size: Some(gl21::COLOR_ARRAY_SIZE),
        stride: gl21::COLOR_ARRAY_STRIDE,
        pointer: gl21::COLOR_ARRAY_POINTER,
    },
    ArrayInfo {
        name: gl21::NORMAL_ARRAY,
        buffer_binding: gl21::NORMAL_ARRAY_BUFFER_BINDING,
        size: None,
        stride: gl21::NORMAL_ARRAY_STRIDE,
        pointer: gl21::NORMAL_ARRAY_POINTER,
    },
    ArrayInfo {
        name: gl21::TEXTURE_COORD_ARRAY,
        buffer_binding: gl21::TEXTURE_COORD_ARRAY_BUFFER_BINDING,
        size: Some(gl21::TEXTURE_COORD_ARRAY_SIZE),
        stride: gl21::TEXTURE_COORD_ARRAY_STRIDE,
        pointer: gl21::TEXTURE_COORD_ARRAY_POINTER,
    },
    ArrayInfo {
        name: gl21::VERTEX_ARRAY,
        buffer_binding: gl21::VERTEX_ARRAY_BUFFER_BINDING,
        size: Some(gl21::VERTEX_ARRAY_SIZE),
        stride: gl21::VERTEX_ARRAY_STRIDE,
        pointer: gl21::VERTEX_ARRAY_POINTER,
    },
];

/// Table of `glGet` parameters shared by OpenGL ES 1.1 and OpenGL 2.1.
const GET_PARAMS: ParamTable = ParamTable(&[
    (gl21::ACTIVE_TEXTURE, ParamType::Int, 1),
    (gl21::ALIASED_POINT_SIZE_RANGE, ParamType::Float, 2),
    (gl21::ALIASED_LINE_WIDTH_RANGE, ParamType::Float, 2),
    (gl21::ALPHA_BITS, ParamType::Int, 1),
    (gl21::ALPHA_TEST, ParamType::Boolean, 1),
    (gl21::ALPHA_TEST_FUNC, ParamType::Int, 1),
    // TODO: ALPHA_TEST_REF (has special type conversion behavior)
    (gl21::ARRAY_BUFFER_BINDING, ParamType::Int, 1),
    (gl21::BLEND, ParamType::Boolean, 1),
    (gl21::BLEND_DST, ParamType::Int, 1),
    (gl21::BLEND_SRC, ParamType::Int, 1),
    (gl21::BLUE_BITS, ParamType::Int, 1),
    (gl21::CLIENT_ACTIVE_TEXTURE, ParamType::Int, 1),
    // TODO: arbitrary number of clip planes?
    (gl21::CLIP_PLANE0, ParamType::Boolean, 1),
    (gl21::CLIP_PLANE1, ParamType::Boolean, 1),
    (gl21::CLIP_PLANE2, ParamType::Boolean, 1),
    (gl21::CLIP_PLANE3, ParamType::Boolean, 1),
    (gl21::CLIP_PLANE4, ParamType::Boolean, 1),
    (gl21::CLIP_PLANE5, ParamType::Boolean, 1),
    (gl21::COLOR_ARRAY, ParamType::Boolean, 1),
    (gl21::COLOR_ARRAY_BUFFER_BINDING, ParamType::Int, 1),
    (gl21::COLOR_ARRAY_SIZE, ParamType::Int, 1),
    (gl21::COLOR_ARRAY_STRIDE, ParamType::Int, 1),
    (gl21::COLOR_ARRAY_TYPE, ParamType::Int, 1),
    (gl21::COLOR_CLEAR_VALUE, ParamType::FloatSpecial, 4), // TODO correct type
    (gl21::COLOR_LOGIC_OP, ParamType::Boolean, 1),
    (gl21::COLOR_MATERIAL, ParamType::Boolean, 1),
    (gl21::COLOR_WRITEMASK, ParamType::Boolean, 4),
    // TODO: COMPRESSED_TEXTURE_FORMATS (needs to return only supported formats)
    (gl21::CULL_FACE, ParamType::Boolean, 1),
    (gl21::CULL_FACE_MODE, ParamType::Int, 1),
    (gl21::CURRENT_COLOR, ParamType::FloatSpecial, 4), // TODO correct type
    // TODO: CURRENT_NORMAL (has special type conversion behavior)
    (gl21::CURRENT_TEXTURE_COORDS, ParamType::Float, 4),
    (gl21::DEPTH_BITS, ParamType::Int, 1),
    // TODO: DEPTH_CLEAR_VALUE (has special type conversion behavior)
    (gl21::DEPTH_FUNC, ParamType::Int, 1),
    // TODO: DEPTH_RANGE (has special type conversion behavior)
    (gl21::DEPTH_TEST, ParamType::Boolean, 1),
    (gl21::DEPTH_WRITEMASK, ParamType::Boolean, 1),
    (gl21::DITHER, ParamType::Boolean, 1),
    (gl21::ELEMENT_ARRAY_BUFFER_BINDING, ParamType::Int, 1),
    (gl21::FOG, ParamType::Boolean, 1),
    // TODO: FOG_COLOR (has special type conversion behavior)
    (gl21::FOG_HINT, ParamType::Int, 1),
    (gl21::FOG_MODE, ParamType::Int, 1),
    (gl21::FOG_DENSITY, ParamType::Float, 1),
    (gl21::FOG_START, ParamType::Float, 1),
    (gl21::FOG_END, ParamType::Float, 1),
    (gl21::FRONT_FACE, ParamType::Int, 1),
    (gl21::GREEN_BITS, ParamType::Int, 1),
    // TODO: IMPLEMENTATION_COLOR_READ_FORMAT_OES? (not shared)
    // TODO: IMPLEMENTATION_COLOR_READ_TYPE_OES? (not shared)
    // TODO: LIGHT_MODEL_AMBIENT (has special type conversion behavior)
    (gl21::LIGHT_MODEL_TWO_SIDE, ParamType::Boolean, 1),
    // TODO: arbitrary number of lights?
    (gl21::LIGHT0, ParamType::Boolean, 1),
    (gl21::LIGHT1, ParamType::Boolean, 1),
    (gl21::LIGHT2, ParamType::Boolean, 1),
    (gl21::LIGHT3, ParamType::Boolean, 1),
    (gl21::LIGHT4, ParamType::Boolean, 1),
    (gl21::LIGHT5, ParamType::Boolean, 1),
    (gl21::LIGHT6, ParamType::Boolean, 1),
    (gl21::LIGHT7, ParamType::Boolean, 1),
    (gl21::LIGHTING, ParamType::Boolean, 1),
    (gl21::LINE_SMOOTH, ParamType::Boolean, 1),
    (gl21::LINE_SMOOTH_HINT, ParamType::Int, 1),
    (gl21::LINE_WIDTH, ParamType::Float, 1),
    (gl21::LOGIC_OP_MODE, ParamType::Int, 1),
    (gl21::MATRIX_MODE, ParamType::Int, 1),
    (gl21::MAX_CLIP_PLANES, ParamType::Int, 1),
    (gl21::MAX_LIGHTS, ParamType::Int, 1),
    (gl21::MAX_MODELVIEW_STACK_DEPTH, ParamType::Int, 1),
    (gl21::MAX_PROJECTION_STACK_DEPTH, ParamType::Int, 1),
    (gl21::MAX_TEXTURE_MAX_ANISOTROPY_EXT, ParamType::Float, 1),
    (gl21::MAX_TEXTURE_SIZE, ParamType::Int, 1),
    (gl21::MAX_TEXTURE_STACK_DEPTH, ParamType::Int, 1),
    (gl21::MAX_TEXTURE_UNITS, ParamType::Int, 1),
    (gl21::MAX_VIEWPORT_DIMS, ParamType::Int, 1),
    (gl21::MODELVIEW_MATRIX, ParamType::Float, 16),
    (gl21::MODELVIEW_STACK_DEPTH, ParamType::Int, 1),
    (gl21::MULTISAMPLE, ParamType::Boolean, 1),
    (gl21::NORMAL_ARRAY, ParamType::Boolean, 1),
    (gl21::NORMAL_ARRAY_BUFFER_BINDING, ParamType::Int, 1),
    (gl21::NORMAL_ARRAY_STRIDE, ParamType::Int, 1),
    (gl21::NORMAL_ARRAY_TYPE, ParamType::Int, 1),
    (gl21::NORMALIZE, ParamType::Boolean, 1),
    (gl21::PACK_ALIGNMENT, ParamType::Int, 1),
    (gl21::PERSPECTIVE_CORRECTION_HINT, ParamType::Int, 1),
    (gl21::POINT_DISTANCE_ATTENUATION, ParamType::Float, 3),
    (gl21::POINT_FADE_THRESHOLD_SIZE, ParamType::Float, 1),
    (gl21::POINT_SIZE, ParamType::Float, 1),
    // TODO: POINT_SIZE_ARRAY_OES etc? (not shared)
    (gl21::POINT_SIZE_MAX, ParamType::Float, 1),
    (gl21::POINT_SIZE_MIN, ParamType::Float, 1),
    (gl21::POINT_SIZE_RANGE, ParamType::Float, 2),
    (gl21::POINT_SMOOTH, ParamType::Boolean, 2),
    (gl21::POINT_SMOOTH_HINT, ParamType::Int, 2),
    (gl21::POINT_SPRITE, ParamType::Boolean, 1),
    (gl21::POLYGON_OFFSET_FACTOR, ParamType::Float, 1),
    (gl21::POLYGON_OFFSET_FILL, ParamType::Boolean, 1),
    (gl21::POLYGON_OFFSET_UNITS, ParamType::Float, 1),
    (gl21::PROJECTION_MATRIX, ParamType::Float, 16),
    (gl21::PROJECTION_STACK_DEPTH, ParamType::Int, 1),
    (gl21::RED_BITS, ParamType::Int, 1),
    (gl21::RESCALE_NORMAL, ParamType::Boolean, 1),
    (gl21::SAMPLE_ALPHA_TO_COVERAGE, ParamType::Boolean, 1),
    (gl21::SAMPLE_ALPHA_TO_ONE, ParamType::Boolean, 1),
    (gl21::SAMPLE_BUFFERS, ParamType::Int, 1),
    (gl21::SAMPLE_COVERAGE, ParamType::Boolean, 1),
    (gl21::SAMPLE_COVERAGE_INVERT, ParamType::Boolean, 1),
    (gl21::SAMPLE_COVERAGE_VALUE, ParamType::Float, 1),
    (gl21::SAMPLES, ParamType::Int, 1),
    (gl21::SCISSOR_BOX, ParamType::Int, 4),
    (gl21::SCISSOR_TEST, ParamType::Boolean, 1),
    (gl21::SHADE_MODEL, ParamType::Int, 1),
    (gl21::SMOOTH_LINE_WIDTH_RANGE, ParamType::Float, 2),
    (gl21::SMOOTH_POINT_SIZE_RANGE, ParamType::Float, 2),
    (gl21::STENCIL_BITS, ParamType::Int, 1),
    (gl21::STENCIL_CLEAR_VALUE, ParamType::Int, 1),
    (gl21::STENCIL_FAIL, ParamType::Int, 1),
    (gl21::STENCIL_FUNC, ParamType::Int, 1),
    (gl21::STENCIL_PASS_DEPTH_FAIL, ParamType::Int, 1),
    (gl21::STENCIL_PASS_DEPTH_PASS, ParamType::Int, 1),
    (gl21::STENCIL_REF, ParamType::Int, 1),
    (gl21::STENCIL_TEST, ParamType::Boolean, 1),
    (gl21::STENCIL_VALUE_MASK, ParamType::Int, 1),
    (gl21::STENCIL_WRITEMASK, ParamType::Int, 1),
    (gl21::SUBPIXEL_BITS, ParamType::Int, 1),
    (gl21::TEXTURE_2D, ParamType::Boolean, 1),
    (gl21::TEXTURE_BINDING_2D, ParamType::Int, 1),
    (gl21::TEXTURE_COORD_ARRAY, ParamType::Boolean, 1),
    (gl21::TEXTURE_COORD_ARRAY_BUFFER_BINDING, ParamType::Int, 1),
    (gl21::TEXTURE_COORD_ARRAY_SIZE, ParamType::Int, 1),
    (gl21::TEXTURE_COORD_ARRAY_STRIDE, ParamType::Int, 1),
    (gl21::TEXTURE_COORD_ARRAY_TYPE, ParamType::Int, 1),
    (gl21::TEXTURE_MATRIX, ParamType::Float, 16),
    (gl21::TEXTURE_STACK_DEPTH, ParamType::Int, 1),
    (gl21::UNPACK_ALIGNMENT, ParamType::Int, 1),
    (gl21::VIEWPORT, ParamType::Int, 4),
    (gl21::VERTEX_ARRAY, ParamType::Boolean, 1),
    (gl21::VERTEX_ARRAY_BUFFER_BINDING, ParamType::Int, 1),
    (gl21::VERTEX_ARRAY_SIZE, ParamType::Int, 1),
    (gl21::VERTEX_ARRAY_STRIDE, ParamType::Int, 1),
    (gl21::VERTEX_ARRAY_TYPE, ParamType::Int, 1),
    // OES_framebuffer_object -> EXT_framebuffer_object
    (gl21::FRAMEBUFFER_BINDING_EXT, ParamType::Int, 1),
    (gl21::RENDERBUFFER_BINDING_EXT, ParamType::Int, 1),
    // EXT_texture_lod_bias
    (gl21::MAX_TEXTURE_LOD_BIAS_EXT, ParamType::Float, 1),
    // OES_matrix_palette -> ARB_matrix_palette
    (gl21::MAX_PALETTE_MATRICES_ARB, ParamType::Int, 1),
    // OES_matrix_palette -> ARB_vertex_blend
    (gl21::MAX_VERTEX_UNITS_ARB, ParamType::Int, 1),
]);

const UNSUPPORTED_GET_PARAMS: ParamTable = ParamTable(&[
    (gl21::COMPRESSED_TEXTURE_FORMATS, ParamType::Int, 0), // Dynamically sized
]);

const POINT_PARAMS: ParamTable = ParamTable(&[
    (gl21::POINT_SIZE_MIN, ParamType::Float, 1),
    (gl21::POINT_SIZE_MAX, ParamType::Float, 1),
    (gl21::POINT_DISTANCE_ATTENUATION, ParamType::Float, 3),
    (gl21::POINT_FADE_THRESHOLD_SIZE, ParamType::Float, 1),
    (gl21::POINT_SMOOTH, ParamType::Boolean, 1),
]);

/// Table of `glFog` parameters shared by OpenGL ES 1.1 and OpenGL 2.1.
const FOG_PARAMS: ParamTable = ParamTable(&[
    // Despite only having f, fv, x and xv setters in OpenGL ES 1.1, this is
    // an integer! (You're meant to use the x/xv setter.)
    (gl21::FOG_MODE, ParamType::Int, 1),
    (gl21::FOG_DENSITY, ParamType::Float, 1),
    (gl21::FOG_START, ParamType::Float, 1),
    (gl21::FOG_END, ParamType::Float, 1),
    (gl21::FOG_COLOR, ParamType::FloatSpecial, 4), // TODO correct type
]);

/// Table of `glLight` parameters shared by OpenGL ES 1.1 and OpenGL 2.1.
const LIGHT_PARAMS: ParamTable = ParamTable(&[
    (gl21::AMBIENT, ParamType::Float, 4),
    (gl21::DIFFUSE, ParamType::Float, 4),
    (gl21::SPECULAR, ParamType::Float, 4),
    (gl21::POSITION, ParamType::Float, 4),
    (gl21::SPOT_CUTOFF, ParamType::Float, 1),
    (gl21::SPOT_DIRECTION, ParamType::Float, 3),
    (gl21::SPOT_EXPONENT, ParamType::Float, 1),
    (gl21::CONSTANT_ATTENUATION, ParamType::Float, 1),
    (gl21::LINEAR_ATTENUATION, ParamType::Float, 1),
    (gl21::QUADRATIC_ATTENUATION, ParamType::Float, 1),
]);

const LIGHT_MODEL_PARAMS: ParamTable = ParamTable(&[
    (gl21::LIGHT_MODEL_AMBIENT, ParamType::Float, 4),
    (gl21::LIGHT_MODEL_TWO_SIDE, ParamType::Boolean, 1),
]);

const MATERIAL_PARAMS: ParamTable = ParamTable(&[
    (gl21::AMBIENT, ParamType::Float, 4),
    (gl21::DIFFUSE, ParamType::Float, 4),
    (gl21::SPECULAR, ParamType::Float, 4),
    (gl21::EMISSION, ParamType::Float, 4),
    (gl21::SHININESS, ParamType::Float, 1),
    (gl21::AMBIENT_AND_DIFFUSE, ParamType::Float, 4),
]);

const TEX_ENV_PARAMS: ParamTable = ParamTable(&[
    (gl21::TEXTURE_ENV_MODE, ParamType::Int, 1),
    (gl21::TEXTURE_ENV_COLOR, ParamType::FloatSpecial, 4), // TODO correct type
    (gl21::COMBINE_RGB, ParamType::Int, 1),
    (gl21::COMBINE_ALPHA, ParamType::Int, 1),
    (gl21::SRC0_RGB, ParamType::Int, 1),
    (gl21::SRC1_RGB, ParamType::Int, 1),
    (gl21::SRC2_RGB, ParamType::Int, 1),
    (gl21::SRC0_ALPHA, ParamType::Int, 1),
    (gl21::SRC1_ALPHA, ParamType::Int, 1),
    (gl21::SRC2_ALPHA, ParamType::Int, 1),
    (gl21::OPERAND0_RGB, ParamType::Int, 1),
    (gl21::OPERAND1_RGB, ParamType::Int, 1),
    (gl21::OPERAND2_RGB, ParamType::Int, 1),
    (gl21::OPERAND0_ALPHA, ParamType::Int, 1),
    (gl21::OPERAND1_ALPHA, ParamType::Int, 1),
    (gl21::OPERAND2_ALPHA, ParamType::Int, 1),
    (gl21::RGB_SCALE, ParamType::Float, 1),
    (gl21::ALPHA_SCALE, ParamType::Float, 1),
    (gl21::COORD_REPLACE, ParamType::Boolean, 1),
]);

const TEX_PARAMETER_PARAMS: ParamTable = ParamTable(&[
    (gl21::TEXTURE_MAG_FILTER, ParamType::Int, 1),
    (gl21::TEXTURE_MIN_FILTER, ParamType::Int, 1),
    (gl21::TEXTURE_WRAP_S, ParamType::Int, 1),
    (gl21::TEXTURE_WRAP_T, ParamType::Int, 1),
    (gl21::GENERATE_MIPMAP, ParamType::Boolean, 1),
    (gl21::TEXTURE_MAX_ANISOTROPY_EXT, ParamType::Float, 1),
]);

pub struct GLES1OnGL2 {
    gl: GLContext,
}

impl GLES1OnGL2 {
    pub fn description() -> &'static str {
        "OpenGL ES 1.1 on OpenGL 2.1 compatibility profile"
    }

    pub fn new(window: &mut Window) -> Result<Self, String> {
        let gl = window.gl_create_context(GLVersion::V2_1Compat);
        gl.make_current();

        unsafe {
            let mut extensions = HashSet::new();
            let extension = gl21::GetString(gl21::EXTENSIONS);
            if !extension.is_null() {
                let extension = CStr::from_ptr(extension as *const i8)
                    .to_str()
                    .unwrap_or_default();
                for ext in extension.split(' ') {
                    extensions.insert(ext.to_string());
                }
            }

            if !extensions.contains("GL_EXT_framebuffer_object") {
                return Err(
                    "Required OpenGL extension GL_EXT_framebuffer_object is missing.".to_string()
                );
            }
        }

        Ok(Self { gl })
    }
}

/// Type alias so that `gles.rs` can import `GLES1OnGL2Context` as expected.
pub type GLES1OnGL2Context = GLES1OnGL2;

impl GLES for GLES1OnGL2 {
    unsafe fn AlphaFunc(&mut self, func: GLenum, ref_: GLclampf) {
        gl21::AlphaFunc(func, ref_)
    }
    unsafe fn AlphaFuncx(&mut self, func: GLenum, ref_: GLfixed) {
        gl21::AlphaFunc(func, fixed_to_float(ref_))
    }
    unsafe fn BindTexture(&mut self, target: GLenum, texture: GLuint) {
        gl21::BindTexture(target, texture)
    }
    unsafe fn BlendFunc(&mut self, sfactor: GLenum, dfactor: GLenum) {
        gl21::BlendFunc(sfactor, dfactor)
    }
    unsafe fn Clear(&mut self, mask: GLbitfield) {
        gl21::Clear(mask)
    }
    unsafe fn ClearColor(&mut self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        gl21::ClearColor(red, green, blue, alpha)
    }
    unsafe fn ClearColorx(&mut self, red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed) {
        gl21::ClearColor(
            fixed_to_float(red),
            fixed_to_float(green),
            fixed_to_float(blue),
            fixed_to_float(alpha),
        )
    }
    unsafe fn ClearDepthf(&mut self, depth: GLclampf) {
        gl21::ClearDepth(depth as _)
    }
    unsafe fn ClearDepthx(&mut self, depth: GLfixed) {
        gl21::ClearDepth(fixed_to_float(depth) as _)
    }
    unsafe fn ClearStencil(&mut self, s: GLint) {
        gl21::ClearStencil(s)
    }
    unsafe fn ClientActiveTexture(&mut self, texture: GLenum) {
        gl21::ClientActiveTexture(texture)
    }
    unsafe fn Color4f(&mut self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
        gl21::Color4f(red, green, blue, alpha)
    }
    unsafe fn Color4ub(&mut self, red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) {
        gl21::Color4ub(red, green, blue, alpha)
    }
    unsafe fn Color4x(&mut self, red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed) {
        gl21::Color4f(
            fixed_to_float(red),
            fixed_to_float(green),
            fixed_to_float(blue),
            fixed_to_float(alpha),
        )
    }
    unsafe fn ColorMask(&mut self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        gl21::ColorMask(red, green, blue, alpha)
    }
    unsafe fn ColorPointer(
        &mut self,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        gl21::ColorPointer(size, type_, stride, pointer)
    }
    unsafe fn CopyTexImage2D(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        gl21::CopyTexImage2D(target, level, internalformat, x, y, width, height, border)
    }
    unsafe fn CopyTexSubImage2D(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        gl21::CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
    }
    unsafe fn CullFace(&mut self, mode: GLenum) {
        gl21::CullFace(mode)
    }
    unsafe fn DeleteTextures(&mut self, n: GLsizei, textures: *const GLuint) {
        gl21::DeleteTextures(n, textures)
    }
    unsafe fn DepthFunc(&mut self, func: GLenum) {
        gl21::DepthFunc(func)
    }
    unsafe fn DepthMask(&mut self, flag: GLboolean) {
        gl21::DepthMask(flag)
    }
    unsafe fn DepthRangef(&mut self, zNear: GLclampf, zFar: GLclampf) {
        gl21::DepthRange(zNear as _, zFar as _)
    }
    unsafe fn DepthRangex(&mut self, zNear: GLfixed, zFar: GLfixed) {
        gl21::DepthRange(fixed_to_float(zNear) as _, fixed_to_float(zFar) as _)
    }
    unsafe fn Disable(&mut self, cap: GLenum) {
        if UNSUPPORTED_CAPABILITIES.contains(&cap) {
            log!("Warning: Disabling unsupported capability 0x{:04x}", cap);
            return;
        }
        gl21::Disable(cap)
    }
    unsafe fn DisableClientState(&mut self, array: GLenum) {
        gl21::DisableClientState(array)
    }
    unsafe fn DrawArrays(&mut self, mode: GLenum, first: GLint, count: GLsizei) {
        gl21::DrawArrays(mode, first, count)
    }
    unsafe fn DrawElements(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
    ) {
        gl21::DrawElements(mode, count, type_, indices)
    }
    unsafe fn Enable(&mut self, cap: GLenum) {
        if UNSUPPORTED_CAPABILITIES.contains(&cap) {
            log!("Warning: Enabling unsupported capability 0x{:04x}", cap);
            return;
        }
        gl21::Enable(cap)
    }
    unsafe fn EnableClientState(&mut self, array: GLenum) {
        gl21::EnableClientState(array)
    }
    unsafe fn Finish(&mut self) {
        gl21::Finish()
    }
    unsafe fn Flush(&mut self) {
        gl21::Flush()
    }
    unsafe fn Fogf(&mut self, pname: GLenum, param: GLfloat) {
        gl21::Fogf(pname, param)
    }
    unsafe fn Fogfv(&mut self, pname: GLenum, params: *const GLfloat) {
        gl21::Fogfv(pname, params)
    }
    unsafe fn Fogx(&mut self, pname: GLenum, param: GLfixed) {
        if FOG_PARAMS.is_int(pname) {
            gl21::Fogi(pname, param)
        } else {
            gl21::Fogf(pname, fixed_to_float(param))
        }
    }
    unsafe fn Fogxv(&mut self, pname: GLenum, params: *const GLfixed) {
        let converted = FOG_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::Fogfv(pname, converted.as_ptr())
    }
    unsafe fn FrontFace(&mut self, mode: GLenum) {
        gl21::FrontFace(mode)
    }
    unsafe fn Frustumf(
        &mut self,
        left: GLfloat,
        right: GLfloat,
        bottom: GLfloat,
        top: GLfloat,
        zNear: GLfloat,
        zFar: GLfloat,
    ) {
        gl21::Frustum(left as _, right as _, bottom as _, top as _, zNear as _, zFar as _)
    }
    unsafe fn Frustumx(
        &mut self,
        left: GLfixed,
        right: GLfixed,
        bottom: GLfixed,
        top: GLfixed,
        zNear: GLfixed,
        zFar: GLfixed,
    ) {
        gl21::Frustum(
            fixed_to_float(left) as _,
            fixed_to_float(right) as _,
            fixed_to_float(bottom) as _,
            fixed_to_float(top) as _,
            fixed_to_float(zNear) as _,
            fixed_to_float(zFar) as _,
        )
    }
    unsafe fn GenTextures(&mut self, n: GLsizei, textures: *mut GLuint) {
        gl21::GenTextures(n, textures)
    }
    unsafe fn GetError(&mut self) -> GLenum {
        gl21::GetError()
    }
    unsafe fn GetIntegerv(&mut self, pname: GLenum, params: *mut GLint) {
        if UNSUPPORTED_GET_PARAMS.contains(pname) {
            log!("Warning: glGetIntegerv unsupported param 0x{:04x}", pname);
            return;
        }
        gl21::GetIntegerv(pname, params)
    }
    unsafe fn GetString(&mut self, name: GLenum) -> *const GLubyte {
        match name {
            gl21::EXTENSIONS => {
                // TODO: only return supported extensions
                gl21::GetString(name)
            }
            _ => gl21::GetString(name),
        }
    }
    unsafe fn Hint(&mut self, target: GLenum, mode: GLenum) {
        gl21::Hint(target, mode)
    }
    unsafe fn Lightf(&mut self, light: GLenum, pname: GLenum, param: GLfloat) {
        gl21::Lightf(light, pname, param)
    }
    unsafe fn Lightfv(&mut self, light: GLenum, pname: GLenum, params: *const GLfloat) {
        gl21::Lightfv(light, pname, params)
    }
    unsafe fn Lightx(&mut self, light: GLenum, pname: GLenum, param: GLfixed) {
        gl21::Lightf(light, pname, fixed_to_float(param))
    }
    unsafe fn Lightxv(&mut self, light: GLenum, pname: GLenum, params: *const GLfixed) {
        let converted = LIGHT_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::Lightfv(light, pname, converted.as_ptr())
    }
    unsafe fn LightModelf(&mut self, pname: GLenum, param: GLfloat) {
        gl21::LightModelf(pname, param)
    }
    unsafe fn LightModelfv(&mut self, pname: GLenum, params: *const GLfloat) {
        gl21::LightModelfv(pname, params)
    }
    unsafe fn LightModelx(&mut self, pname: GLenum, param: GLfixed) {
        gl21::LightModelf(pname, fixed_to_float(param))
    }
    unsafe fn LightModelxv(&mut self, pname: GLenum, params: *const GLfixed) {
        let converted = LIGHT_MODEL_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::LightModelfv(pname, converted.as_ptr())
    }
    unsafe fn LineWidth(&mut self, width: GLfloat) {
        gl21::LineWidth(width)
    }
    unsafe fn LineWidthx(&mut self, width: GLfixed) {
        gl21::LineWidth(fixed_to_float(width))
    }
    unsafe fn LoadIdentity(&mut self) {
        gl21::LoadIdentity()
    }
    unsafe fn LoadMatrixf(&mut self, m: *const GLfloat) {
        gl21::LoadMatrixf(m)
    }
    unsafe fn LoadMatrixx(&mut self, m: *const GLfixed) {
        gl21::LoadMatrixf(matrix_fixed_to_float(m).as_ptr())
    }
    unsafe fn LogicOp(&mut self, opcode: GLenum) {
        gl21::LogicOp(opcode)
    }
    unsafe fn Materialf(&mut self, face: GLenum, pname: GLenum, param: GLfloat) {
        gl21::Materialf(face, pname, param)
    }
    unsafe fn Materialfv(&mut self, face: GLenum, pname: GLenum, params: *const GLfloat) {
        gl21::Materialfv(face, pname, params)
    }
    unsafe fn Materialx(&mut self, face: GLenum, pname: GLenum, param: GLfixed) {
        gl21::Materialf(face, pname, fixed_to_float(param))
    }
    unsafe fn Materialxv(&mut self, face: GLenum, pname: GLenum, params: *const GLfixed) {
        let converted = MATERIAL_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::Materialfv(face, pname, converted.as_ptr())
    }
    unsafe fn MatrixMode(&mut self, mode: GLenum) {
        gl21::MatrixMode(mode)
    }
    unsafe fn MultiTexCoord4f(&mut self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) {
        gl21::MultiTexCoord4f(target, s, t, r, q)
    }
    unsafe fn MultiTexCoord4x(&mut self, target: GLenum, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed) {
        gl21::MultiTexCoord4f(
            target,
            fixed_to_float(s),
            fixed_to_float(t),
            fixed_to_float(r),
            fixed_to_float(q),
        )
    }
    unsafe fn MultMatrixf(&mut self, m: *const GLfloat) {
        gl21::MultMatrixf(m)
    }
    unsafe fn MultMatrixx(&mut self, m: *const GLfixed) {
        gl21::MultMatrixf(matrix_fixed_to_float(m).as_ptr())
    }
    unsafe fn Normal3f(&mut self, nx: GLfloat, ny: GLfloat, nz: GLfloat) {
        gl21::Normal3f(nx, ny, nz)
    }
    unsafe fn Normal3x(&mut self, nx: GLfixed, ny: GLfixed, nz: GLfixed) {
        gl21::Normal3f(fixed_to_float(nx), fixed_to_float(ny), fixed_to_float(nz))
    }
    unsafe fn NormalPointer(&mut self, type_: GLenum, stride: GLsizei, pointer: *const GLvoid) {
        gl21::NormalPointer(type_, stride, pointer)
    }
    unsafe fn Orthof(
        &mut self,
        left: GLfloat,
        right: GLfloat,
        bottom: GLfloat,
        top: GLfloat,
        zNear: GLfloat,
        zFar: GLfloat,
    ) {
        gl21::Ortho(left as _, right as _, bottom as _, top as _, zNear as _, zFar as _)
    }
    unsafe fn Orthox(
        &mut self,
        left: GLfixed,
        right: GLfixed,
        bottom: GLfixed,
        top: GLfixed,
        zNear: GLfixed,
        zFar: GLfixed,
    ) {
        gl21::Ortho(
            fixed_to_float(left) as _,
            fixed_to_float(right) as _,
            fixed_to_float(bottom) as _,
            fixed_to_float(top) as _,
            fixed_to_float(zNear) as _,
            fixed_to_float(zFar) as _,
        )
    }
    unsafe fn PixelStorei(&mut self, pname: GLenum, param: GLint) {
        gl21::PixelStorei(pname, param)
    }
    unsafe fn PointParameterf(&mut self, pname: GLenum, param: GLfloat) {
        gl21::PointParameterf(pname, param)
    }
    unsafe fn PointParameterfv(&mut self, pname: GLenum, params: *const GLfloat) {
        gl21::PointParameterfv(pname, params)
    }
    unsafe fn PointParameterx(&mut self, pname: GLenum, param: GLfixed) {
        gl21::PointParameterf(pname, fixed_to_float(param))
    }
    unsafe fn PointParameterxv(&mut self, pname: GLenum, params: *const GLfixed) {
        let converted = POINT_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::PointParameterfv(pname, converted.as_ptr())
    }
    unsafe fn PointSize(&mut self, size: GLfloat) {
        gl21::PointSize(size)
    }
    unsafe fn PointSizex(&mut self, size: GLfixed) {
        gl21::PointSize(fixed_to_float(size))
    }
    unsafe fn PolygonOffset(&mut self, factor: GLfloat, units: GLfloat) {
        gl21::PolygonOffset(factor, units)
    }
    unsafe fn PolygonOffsetx(&mut self, factor: GLfixed, units: GLfixed) {
        gl21::PolygonOffset(fixed_to_float(factor), fixed_to_float(units))
    }
    unsafe fn PopMatrix(&mut self) {
        gl21::PopMatrix()
    }
    unsafe fn PushMatrix(&mut self) {
        gl21::PushMatrix()
    }
    unsafe fn ReadPixels(
        &mut self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut GLvoid,
    ) {
        gl21::ReadPixels(x, y, width, height, format, type_, pixels)
    }
    unsafe fn RenderbufferStorageOES(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        gl21::RenderbufferStorageEXT(target, internalformat, width, height)
    }
    unsafe fn Rotatef(&mut self, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) {
        gl21::Rotatef(angle, x, y, z)
    }
    unsafe fn Rotatex(&mut self, angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed) {
        gl21::Rotatef(fixed_to_float(angle), fixed_to_float(x), fixed_to_float(y), fixed_to_float(z))
    }
    unsafe fn SampleCoverage(&mut self, value: GLclampf, invert: GLboolean) {
        gl21::SampleCoverage(value, invert)
    }
    unsafe fn SampleCoveragex(&mut self, value: GLfixed, invert: GLboolean) {
        gl21::SampleCoverage(fixed_to_float(value), invert)
    }
    unsafe fn Scalef(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) {
        gl21::Scalef(x, y, z)
    }
    unsafe fn Scalex(&mut self, x: GLfixed, y: GLfixed, z: GLfixed) {
        gl21::Scalef(fixed_to_float(x), fixed_to_float(y), fixed_to_float(z))
    }
    unsafe fn Scissor(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        gl21::Scissor(x, y, width, height)
    }
    unsafe fn ShadeModel(&mut self, mode: GLenum) {
        gl21::ShadeModel(mode)
    }
    unsafe fn StencilFunc(&mut self, func: GLenum, ref_: GLint, mask: GLuint) {
        gl21::StencilFunc(func, ref_, mask)
    }
    unsafe fn StencilMask(&mut self, mask: GLuint) {
        gl21::StencilMask(mask)
    }
    unsafe fn StencilOp(&mut self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        gl21::StencilOp(fail, zfail, zpass)
    }
    unsafe fn TexCoordPointer(
        &mut self,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        gl21::TexCoordPointer(size, type_, stride, pointer)
    }
    unsafe fn TexEnvf(&mut self, target: GLenum, pname: GLenum, param: GLfloat) {
        gl21::TexEnvf(target, pname, param)
    }
    unsafe fn TexEnvfv(&mut self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        gl21::TexEnvfv(target, pname, params)
    }
    unsafe fn TexEnvi(&mut self, target: GLenum, pname: GLenum, param: GLint) {
        gl21::TexEnvi(target, pname, param)
    }
    unsafe fn TexEnviv(&mut self, target: GLenum, pname: GLenum, params: *const GLint) {
        gl21::TexEnviv(target, pname, params)
    }
    unsafe fn TexEnvx(&mut self, target: GLenum, pname: GLenum, param: GLfixed) {
        if TEX_ENV_PARAMS.is_int(pname) {
            gl21::TexEnvi(target, pname, param)
        } else {
            gl21::TexEnvf(target, pname, fixed_to_float(param))
        }
    }
    unsafe fn TexEnvxv(&mut self, target: GLenum, pname: GLenum, params: *const GLfixed) {
        let converted = TEX_ENV_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::TexEnvfv(target, pname, converted.as_ptr())
    }
    unsafe fn TexImage2D(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    ) {
        if let Some(format) = PalettedTextureFormat::from_gles(internalformat as _) {
            let (width, height, decoded) = try_decode_pvrtc(width as _, height as _, format, pixels);
            gl21::TexImage2D(
                target,
                level,
                gl21::RGBA8 as _,
                width as _,
                height as _,
                border,
                gl21::RGBA,
                gl21::UNSIGNED_BYTE,
                decoded.as_ptr() as *const _,
            );
        } else {
            gl21::TexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                type_,
                pixels,
            )
        }
    }
    unsafe fn TexParameterf(&mut self, target: GLenum, pname: GLenum, param: GLfloat) {
        gl21::TexParameterf(target, pname, param)
    }
    unsafe fn TexParameterfv(&mut self, target: GLenum, pname: GLenum, params: *const GLfloat) {
        gl21::TexParameterfv(target, pname, params)
    }
    unsafe fn TexParameteri(&mut self, target: GLenum, pname: GLenum, param: GLint) {
        gl21::TexParameteri(target, pname, param)
    }
    unsafe fn TexParameteriv(&mut self, target: GLenum, pname: GLenum, params: *const GLint) {
        gl21::TexParameteriv(target, pname, params)
    }
    unsafe fn TexParameterx(&mut self, target: GLenum, pname: GLenum, param: GLfixed) {
        if TEX_PARAMETER_PARAMS.is_int(pname) {
            gl21::TexParameteri(target, pname, param)
        } else {
            gl21::TexParameterf(target, pname, fixed_to_float(param))
        }
    }
    unsafe fn TexParameterxv(&mut self, target: GLenum, pname: GLenum, params: *const GLfixed) {
        let converted = TEX_PARAMETER_PARAMS.convert_fixed_to_float_vec(pname, params);
        gl21::TexParameterfv(target, pname, converted.as_ptr())
    }
    unsafe fn TexSubImage2D(
        &mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    ) {
        gl21::TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type_, pixels)
    }
    unsafe fn Translatef(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) {
        gl21::Translatef(x, y, z)
    }
    unsafe fn Translatex(&mut self, x: GLfixed, y: GLfixed, z: GLfixed) {
        gl21::Translatef(fixed_to_float(x), fixed_to_float(y), fixed_to_float(z))
    }
    unsafe fn VertexPointer(
        &mut self,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    ) {
        gl21::VertexPointer(size, type_, stride, pointer)
    }
    unsafe fn Viewport(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        gl21::Viewport(x, y, width, height)
    }
    unsafe fn BindFramebufferOES(&mut self, target: GLenum, framebuffer: GLuint) {
        gl21::BindFramebufferEXT(target, framebuffer)
    }
    unsafe fn BindRenderbufferOES(&mut self, target: GLenum, renderbuffer: GLuint) {
        gl21::BindRenderbufferEXT(target, renderbuffer)
    }
    unsafe fn DeleteFramebuffersOES(&mut self, n: GLsizei, framebuffers: *const GLuint) {
        gl21::DeleteFramebuffersEXT(n, framebuffers)
    }
    unsafe fn DeleteRenderbuffersOES(&mut self, n: GLsizei, renderbuffers: *const GLuint) {
        gl21::DeleteRenderbuffersEXT(n, renderbuffers)
    }
    unsafe fn FramebufferRenderbufferOES(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        gl21::FramebufferRenderbufferEXT(target, attachment, renderbuffertarget, renderbuffer)
    }
    unsafe fn FramebufferTexture2DOES(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        gl21::FramebufferTexture2DEXT(target, attachment, textarget, texture, level)
    }
    unsafe fn GenFramebuffersOES(&mut self, n: GLsizei, framebuffers: *mut GLuint) {
        gl21::GenFramebuffersEXT(n, framebuffers)
    }
    unsafe fn GenRenderbuffersOES(&mut self, n: GLsizei, renderbuffers: *mut GLuint) {
        gl21::GenRenderbuffersEXT(n, renderbuffers)
    }
    unsafe fn GenerateMipmapOES(&mut self, target: GLenum) {
        gl21::GenerateMipmapEXT(target)
    }
    unsafe fn GetBufferParameteriv(&mut self, target: GLenum, pname: GLenum, params: *mut GLint) {
        gl21::GetBufferParameteriv(target, pname, params)
    }
    unsafe fn GetFramebufferAttachmentParameterivOES(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        gl21::GetFramebufferAttachmentParameterivEXT(target, attachment, pname, params)
    }
    unsafe fn GetRenderbufferParameterivOES(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        gl21::GetRenderbufferParameterivEXT(target, pname, params)
    }
    unsafe fn CheckFramebufferStatusOES(&mut self, target: GLenum) -> GLenum {
        gl21::CheckFramebufferStatusEXT(target)
    }
    unsafe fn IsTexture(&mut self, texture: GLuint) -> GLboolean {
        gl21::IsTexture(texture)
    }
    unsafe fn IsBuffer(&mut self, buffer: GLuint) -> GLboolean {
        gl21::IsBuffer(buffer)
    }
    unsafe fn IsFramebufferOES(&mut self, framebuffer: GLuint) -> GLboolean {
        gl21::IsFramebufferEXT(framebuffer)
    }
    unsafe fn IsRenderbufferOES(&mut self, renderbuffer: GLuint) -> GLboolean {
        gl21::IsRenderbufferEXT(renderbuffer)
    }
    unsafe fn BindBuffer(&mut self, target: GLenum, buffer: GLuint) {
        gl21::BindBuffer(target, buffer)
    }
    unsafe fn BufferData(&mut self, target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum) {
        gl21::BufferData(target, size, data, usage)
    }
    unsafe fn BufferSubData(
        &mut self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const GLvoid,
    ) {
        gl21::BufferSubData(target, offset, size, data)
    }
    unsafe fn DeleteBuffers(&mut self, n: GLsizei, buffers: *const GLuint) {
        gl21::DeleteBuffers(n, buffers)
    }
    unsafe fn GenBuffers(&mut self, n: GLsizei, buffers: *mut GLuint) {
        gl21::GenBuffers(n, buffers)
    }

    // ── Methods missing from original impl ──────────────────────────────────

    unsafe fn driver_description(&self) -> String {
        let vendor = gl21::GetString(gl21::VENDOR);
        let renderer = gl21::GetString(gl21::RENDERER);
        let version = gl21::GetString(gl21::VERSION);
        let v = |p: *const u8| {
            if p.is_null() { "<null>".to_string() }
            else { std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy().into_owned() }
        };
        format!("OpenGL 2.1 compat ({} {} {})", v(vendor), v(renderer), v(version))
    }

    unsafe fn IsEnabled(&mut self, cap: GLenum) -> GLboolean {
        gl21::IsEnabled(cap)
    }

    unsafe fn GetBooleanv(&mut self, pname: GLenum, params: *mut GLboolean) {
        gl21::GetBooleanv(pname, params)
    }

    unsafe fn GetFloatv(&mut self, pname: GLenum, params: *mut GLfloat) {
        gl21::GetFloatv(pname, params)
    }

    unsafe fn GetTexEnviv(&mut self, target: GLenum, pname: GLenum, params: *mut GLint) {
        gl21::GetTexEnviv(target, pname, params)
    }

    unsafe fn GetTexEnvfv(&mut self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
        gl21::GetTexEnvfv(target, pname, params)
    }

    unsafe fn GetPointerv(&mut self, pname: GLenum, params: *mut *const GLvoid) {
        gl21::GetPointerv(pname, params as *mut *mut GLvoid)
    }

    unsafe fn ActiveTexture(&mut self, texture: GLenum) {
        gl21::ActiveTexture(texture)
    }

    unsafe fn BlendEquationOES(&mut self, mode: GLenum) {
        gl21::BlendEquation(mode)
    }

    unsafe fn ClipPlanef(&mut self, plane: GLenum, equation: *const GLfloat) {
        let eq = std::slice::from_raw_parts(equation, 4);
        let eq64: [f64; 4] = [eq[0] as f64, eq[1] as f64, eq[2] as f64, eq[3] as f64];
        gl21::ClipPlane(plane, eq64.as_ptr())
    }

    unsafe fn ClipPlanex(&mut self, plane: GLenum, equation: *const GLfixed) {
        let eq = std::slice::from_raw_parts(equation, 4);
        let eq64: [f64; 4] = [
            fixed_to_float(eq[0]) as f64,
            fixed_to_float(eq[1]) as f64,
            fixed_to_float(eq[2]) as f64,
            fixed_to_float(eq[3]) as f64,
        ];
        gl21::ClipPlane(plane, eq64.as_ptr())
    }

    unsafe fn CompressedTexImage2D(
        &mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        data: *const GLvoid,
    ) {
        if let Some(format) = PalettedTextureFormat::from_gles(internalformat as _) {
            let (w, h, decoded) = try_decode_pvrtc(width as _, height as _, format, data);
            gl21::TexImage2D(
                target,
                level,
                gl21::RGBA8 as _,
                w as _,
                h as _,
                border,
                gl21::RGBA,
                gl21::UNSIGNED_BYTE,
                decoded.as_ptr() as *const _,
            );
        } else {
            gl21::CompressedTexImage2D(
                target, level, internalformat, width, height, border, image_size, data,
            )
        }
    }

    unsafe fn MapBufferOES(&mut self, target: GLenum, access: GLenum) -> *mut GLvoid {
        gl21::MapBuffer(target, access)
    }

    unsafe fn UnmapBufferOES(&mut self, target: GLenum) -> GLboolean {
        gl21::UnmapBuffer(target)
    }
}
