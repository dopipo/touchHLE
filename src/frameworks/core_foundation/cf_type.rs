/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFType` (type-generic functions etc).

use super::{CFHashCode, CFIndex};
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::foundation::NSUInteger;
use crate::objc::Class;
use crate::{msg, objc};
use crate::{msg_class, Environment};

pub type CFTypeRef = objc::id;
pub type CFTypeID = CFIndex;

pub fn CFRetain(env: &mut Environment, object: CFTypeRef) -> CFTypeRef {
    // assert!(!object.is_null()); // not allowed, unlike for normal objc objects
    objc::retain(env, object)
}
pub fn CFRelease(env: &mut Environment, object: CFTypeRef) {
    objc::release(env, object);
}

pub fn CFGetRetainCount(env: &mut Environment, object: CFTypeRef) -> CFIndex {
    let count: NSUInteger = msg![env; object retainCount];
    count as CFIndex
}

pub fn CFEqual(
    env: &mut Environment,
    object1: CFTypeRef,
    object2: CFTypeRef,
) -> bool {
    if object1 == object2 {
        return true;
    }
    // TODO: other classes
    let str_class: Class = msg_class![env; NSString class];
    let object1_class: Class = msg![env; object1 class];
    // assert!(msg![env; object1_class isKindOfClass:str_class]);
    let object2_class: Class = msg![env; object2 class];
    // assert!(msg![env; object2_class isKindOfClass:str_class]);
    // TODO: use isEqual: once it is fixed
    msg![env; object1 isEqualToString:object2]
}

pub fn CFHash(env: &mut Environment, object: CFTypeRef) -> CFHashCode {
    msg![env; object hash]
}

/// Returns the type ID for a CF object. touchHLE uses the class pointer as a
/// stable unique-per-class value so that CFGetTypeID(x) == CFStringGetTypeID()
/// works correctly when x is an NSString/CFString.
pub fn CFGetTypeID(env: &mut Environment, cf: CFTypeRef) -> CFTypeID {
    if cf.is_null() {
        log_dbg!("CFGetTypeID: called with null, returning 0");
        return 0;
    }
    let class: Class = msg![env; cf class];
    class.to_bits() as CFTypeID
}

// --- Per-type ID functions ---
// Each returns the class pointer of the corresponding ObjC class so that
// comparisons with CFGetTypeID() are consistent.

pub fn CFStringGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSString class];
    class.to_bits() as CFTypeID
}

pub fn CFDictionaryGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSDictionary class];
    class.to_bits() as CFTypeID
}

pub fn CFArrayGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSArray class];
    class.to_bits() as CFTypeID
}

pub fn CFNumberGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSNumber class];
    class.to_bits() as CFTypeID
}

pub fn CFBooleanGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSNumber class];
    class.to_bits() as CFTypeID
}

pub fn CFDataGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSData class];
    class.to_bits() as CFTypeID
}

pub fn CFURLGetTypeID(env: &mut Environment) -> CFTypeID {
    let class: Class = msg_class![env; NSURL class];
    class.to_bits() as CFTypeID
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFRetain(_)),
    export_c_func!(CFRelease(_)),
    export_c_func!(CFGetRetainCount(_)),
    export_c_func!(CFEqual(_, _)),
    export_c_func!(CFHash(_)),
    export_c_func!(CFGetTypeID(_)),
    export_c_func!(CFStringGetTypeID()),
    export_c_func!(CFDictionaryGetTypeID()),
    export_c_func!(CFArrayGetTypeID()),
    export_c_func!(CFNumberGetTypeID()),
    export_c_func!(CFBooleanGetTypeID()),
    export_c_func!(CFDataGetTypeID()),
    export_c_func!(CFURLGetTypeID()),
];
