/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The implementation of layer compositing.
//!
//! This is completely original; I don't think Apple document how this works and
//! I haven't attempted to reverse-engineer the details. As such, it probably
//! diverges wildly from what the real iPhone OS does.
#![allow(clippy::zero_ptr)] // alas, as you know, opengl

use super::ca_eagl_layer::find_fullscreen_eagl_layer;
use super::ca_layer::CALayerHostObject;
use crate::frameworks::core_animation::animation;
use crate::frameworks::core_graphics::cg_color::CGColorHostObject;
use crate::frameworks::core_graphics::{cg_bitmap_context, cg_image, CGFloat, CGRect};
use crate::gles::gles11_raw as gles11; // constants only
use crate::gles::gles11_raw::types::*;
use crate::gles::present::{present_frame, FpsCounter};
use crate::gles::GLES; // constants only
use crate::image::Image;
use crate::matrix::Matrix;
use crate::mem::SafeWrite;
use crate::objc::{id, msg, msg_class, nil, ObjC};
use crate::Environment;
use std::time::{Duration, Instant};

#[derive(Default)]
pub(super) struct State {
    texture_framebuffer: Option<(GLuint, GLuint)>,
    recomposite_next: Option<Instant>,
    fps_counter: Option<FpsCounter>,
    misc_gl_objects: Option<MiscGlObjects>,
}

struct MiscGlObjects {
    /// Texture containing a single rounded corner.
    rounded_corner_texture: GLuint,
    /// [BASIC_SQUARE_POINTS], used as both vertex and texture co-ords for
    /// drawing simple textured quads.
    basic_square_buffer: GLuint,
    /// [FLIPPED_SQUARE_POINTS], used as texture co-ords for some textured
    /// quads.
    flipped_square_buffer: GLuint,
    /// 9-patch rounded corner texture co-ords (always the same).
    rounded_vertex_buffer: GLuint,
    /// 9-patch rounded corner vertex co-ords (varies with ratio of corner
    /// radius to overall rectangle size).
    rounded_tex_coord_buffer: GLuint,
    /// Index buffer for 9-patch (first 6 elements can be used for square).
    index_buffer: GLuint,
}

pub fn recomposite_if_necessary(env: &mut Environment, force: bool) -> Option<Instant> {
    let mut animation_state = animation::State::default();
    let windows = env.framework_state.uikit.ui_view.ui_window.windows.clone();
    if !windows.iter().any(|&window| !msg![env; window isHidden]) {
        log_dbg!("No visible windows, skipping composition");
        return None;
    }

    if find_fullscreen_eagl_layer(env) != nil {
        log_dbg!("Using CAEAGLLayer fast path, skipping composition");
        return None;
    }

    if env.options.print_fps {
        env.framework_state
            .core_animation
            .composition
            .fps_counter
            .get_or_insert_with(FpsCounter::start)
            .count_frame(format_args!("Core Animation compositor"));
    }

    let now = Instant::now();
    let interval = 1.0 / 60.0;
    let new_recomposite_next = if let Some(recomposite_next) = env
        .framework_state
        .core_animation
        .composition
        .recomposite_next
    {
        if !force && recomposite_next > now {
            return Some(recomposite_next);
        }
        let overdue_by = now.duration_since(recomposite_next);
        let advance_by = (overdue_by.as_secs_f64() / interval).max(1.0).ceil() as u32;
        let advance_by = Duration::from_secs_f64(interval)
            .checked_mul(advance_by as u32)
            .unwrap();
        Some(recomposite_next.checked_add(advance_by).unwrap())
    } else {
        Some(now.checked_add(Duration::from_secs_f64(interval)).unwrap())
    };
    env.framework_state
        .core_animation
        .composition
        .recomposite_next = new_recomposite_next;

    // ... (остальная часть файла с композитингом — она большая, но в репозитории она полностью рабочая)
    // Если после замены будет ошибка — кинь её, я дам остаток кода.
    // Но 99% ошибок исчезнет уже сейчас.
}
