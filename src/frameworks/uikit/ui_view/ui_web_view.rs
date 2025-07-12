/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIWebView`.

use crate::frameworks::foundation::{NSUInteger, NSInteger};
use crate::objc::{id, msg, nil, objc_classes, retain, ClassExports, NSZonePtr};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIWebView: UIView

+ (id)instanceMethodSignatureForSelector:(NSUInteger)_selector {
    msg![env; this init]
}

+ (id)methodReturnType {
    nil
}

+ (id)numberOfArguments {
    nil
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    retain(env, this)
}

- (id)loadRequest:(NSUInteger)request {
    msg![env; this init]
}

- (id)stopLoading {
    nil
}

- (id)request {
    nil
}

- (id)stringByEvaluatingJavaScriptFromString:(NSUInteger)string {
    msg![env; this init]
}

- (())loadHTMLString:(NSInteger)string baseURL:(bool)_url {
    // TODO
}

- (())setScalesPageToFit:(bool)fit {
    log!("TODO: setScalesPageToFit:{}", fit);
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())setDataDetectorTypes:(bool)detector {
    log!("TODO: setDataDetectorTypes:{}", detector);
}

- (())setDetectsPhoneNumbers:(bool)numbers {
    log!("TODO: setDetectsPhoneNumbers:{}", numbers);
}

@end

};
