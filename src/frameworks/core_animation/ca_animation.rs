/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CAAnimation` and its subclasses

use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::core_animation::ca_media_timing_function::kCAMediaTimingFunctionDefault;
use crate::frameworks::core_foundation::time::CFTimeInterval;
use crate::frameworks::foundation::ns_string::{get_static_str, to_rust_string};
use crate::objc::{
    autorelease, id, msg, nil, objc_classes, release, retain, todo_objc_setter, ClassExports,
    HostObject, NSZonePtr,
};
use crate::Environment;
use crate::{impl_HostObject_with_superclass, msg_class, msg_super};

pub type CAMediaTimingFillMode = id; // NSString*
pub const kCAFillModeBackwards: &str = "backwards";
pub const kCAFillModeBoth: &str = "both";
pub const kCAFillModeForwards: &str = "forwards";
pub const kCAFillModeRemoved: &str = "removed";

pub const CONSTANTS: ConstantExports = &[
    // CATransitionType values...
    ("_kCAFillModeBackwards", HostConstant::NSString(kCAFillModeBackwards)),
    ("_kCAFillModeBoth", HostConstant::NSString(kCAFillModeBoth)),
    ("_kCAFillModeForwards", HostConstant::NSString(kCAFillModeForwards)),
    ("_kCAFillModeRemoved", HostConstant::NSString(kCAFillModeRemoved)),
];

pub fn get_animation_start_time(env: &mut Environment, animation: id) -> Option<CFTimeInterval> {
    env.objc.borrow::<CAAnimationHostObject>(animation).started_at
}

struct CAAnimationHostObject {
    removed_on_completion: bool,
    timing_function: id,
    delegate: id,
    autoreverses: bool,
    repeat_count: f32,
    begin_time: CFTimeInterval,
    duration: CFTimeInterval,
    fill_mode: &'static str,
    started_at: Option<CFTimeInterval>,
}
impl HostObject for CAAnimationHostObject {}
impl Default for CAAnimationHostObject {
    fn default() -> Self {
        Self {
            removed_on_completion: true,
            timing_function: nil,
            delegate: nil,
            autoreverses: false,
            repeat_count: 0.0,
            begin_time: 0.0,
            duration: 0.0,
            fill_mode: kCAFillModeRemoved,
            started_at: None,
        }
    }
}

// ... (все классы CAAnimation, CAPropertyAnimation, CABasicAnimation и т.д. — они уже есть в твоём старом файле, но обновлённые)
pub const CLASSES: ClassExports = objc_classes! { /* полный блок классов из твоего документа + обновления */ };
