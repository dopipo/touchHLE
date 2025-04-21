/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFAllocator`. Currently there is no actual support for multiple allocators.

use super::CFTypeRef;
use crate::dyld::{ConstantExports, HostConstant};
use crate::mem::Ptr;

pub type CFAllocatorRef = CFTypeRef;

pub const kCFAllocatorDefault: CFAllocatorRef = Ptr::null();
pub const kCFAllocatorSystemDefault: CFAllocatorRef = Ptr::null();
pub const kCFAllocatorMalloc: CFAllocatorRef = Ptr::null();
pub const kCFAllocatorMallocZone: CFAllocatorRef = Ptr::null();
pub const kCFAllocatorNull: CFAllocatorRef = Ptr::null();
pub const kCFAllocatorUseContext: CFAllocatorRef = Ptr::null();
pub const kCAFillModeRemoved: CFAllocatorRef = Ptr::null();
pub const kCAFillModeForwards: CFAllocatorRef = Ptr::null();
pub const kCATransitionFromBottom: CFAllocatorRef = Ptr::null();
pub const kCATransitionFromTop: CFAllocatorRef = Ptr::null();
pub const kCFBooleanTrue: CFAllocatorRef = Ptr::null();
pub const kSecMatchLimit: CFAllocatorRef = Ptr::null();
pub const kSecAttrServer: CFAllocatorRef = Ptr::null();
pub const kSecAttrAuthenticationTypeDefault: CFAllocatorRef = Ptr::null();
pub const kSecClassInternetPassword: CFAllocatorRef = Ptr::null();
pub const kSecAttrAuthenticationType: CFAllocatorRef = Ptr::null();
pub const UIEdgeInsetsZero: CFAllocatorRef = Ptr::null();
pub const kSecMatchLimitOne: CFAllocatorRef = Ptr::null();
pub const kSecClass: CFAllocatorRef = Ptr::null();
pub const kSecAttrType: CFAllocatorRef = Ptr::null();
pub const kSecValueData: CFAllocatorRef = Ptr::null();

pub const CONSTANTS: ConstantExports = &[
    ("_kCFAllocatorDefault", HostConstant::NullPtr),
    ("_kCFAllocatorSystemDefault", HostConstant::NullPtr),
    ("_kCFAllocatorMalloc", HostConstant::NullPtr),
    ("_kCFAllocatorMallocZone", HostConstant::NullPtr),
    ("_kCFAllocatorNull", HostConstant::NullPtr),
    ("_kCFAllocatorUseContext", HostConstant::NullPtr),
    ("_kCAFillModeRemoved", HostConstant::NullPtr),
    ("_kCAFillModeForwards", HostConstant::NullPtr),
    ("_kCATransitionFromBottom", HostConstant::NullPtr),
    ("_kCATransitionFromTop", HostConstant::NullPtr),
    ("_kCFBooleanTrue", HostConstant::NullPtr),
    ("_kCFTypeArrayCallBacks", HostConstant::NullPtr),
    ("_kSecMatchLimit", HostConstant::NullPtr),
    ("_kSecAttrServer", HostConstant::NullPtr),
    ("_kSecAttrAuthenticationTypeDefault", HostConstant::NullPtr),
    ("_kSecClassInternetPassword", HostConstant::NullPtr),
    ("_kSecAttrAuthenticationType", HostConstant::NullPtr),
    ("_UIEdgeInsetsZero", HostConstant::NullPtr),
    ("_kSecMatchLimitOne", HostConstant::NullPtr),
    ("_kSecAttrAuthenticationType", HostConstant::NullPtr),
    ("_UIEdgeInsetsZero", HostConstant::NullPtr),
    ("_kSecMatchLimitOne", HostConstant::NullPtr),
    ("_kSecClass", HostConstant::NullPtr),
    ("_kSecAttrType", HostConstant::NullPtr),
    ("_kSecValueData", HostConstant::NullPtr),
];
