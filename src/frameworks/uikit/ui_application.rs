/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIApplication` and `UIApplicationMain`.

use super::ui_device::*;
use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::frameworks::foundation::ns_string::{from_rust_string, get_static_str};
use crate::frameworks::foundation::{ns_array, ns_string, NSInteger, NSTimeInterval, NSUInteger};
use crate::mem::MutPtr;
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject,
    NSZonePtr,
};
use crate::window::DeviceOrientation;
use crate::Environment;

#[derive(Default)]
pub struct State {
    shared_application: Option<id>,
    pub(super) status_bar_hidden: bool,
}

struct UIApplicationHostObject {
    delegate: id,
    delegate_is_retained: bool,
}
impl HostObject for UIApplicationHostObject {}

pub type UIInterfaceOrientation = UIDeviceOrientation;

pub const UIInterfaceOrientationPortrait: UIInterfaceOrientation = UIDeviceOrientationPortrait;
pub const UIInterfaceOrientationPortraitUpsideDown: UIInterfaceOrientation = UIDeviceOrientationPortraitUpsideDown;
pub const UIInterfaceOrientationLandscapeLeft: UIInterfaceOrientation = UIDeviceOrientationLandscapeLeft;
pub const UIInterfaceOrientationLandscapeRight: UIInterfaceOrientation = UIDeviceOrientationLandscapeRight;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIApplication: UIResponder

+ (id)sharedApplication {
    env.framework_state.uikit.ui_application.shared_application.unwrap_or(nil)
}

- (id)delegate {
    env.objc.borrow::<UIApplicationHostObject>(this).delegate
}

- (())setDelegate:(id)delegate {
    let host_object = env.objc.borrow_mut::<UIApplicationHostObject>(this);
    if host_object.delegate_is_retained {
        release(env, host_object.delegate);
    }
    host_object.delegate = retain(env, delegate);
    host_object.delegate_is_retained = true;
}

- (bool)isStatusBarHidden {
    env.framework_state.uikit.ui_application.status_bar_hidden
}

- (())setStatusBarHidden:(bool)hidden {
    env.framework_state.uikit.ui_application.status_bar_hidden = hidden;
}

- (())setStatusBarHidden:(bool)hidden animated:(bool)_animated {
    msg![env; this setStatusBarHidden:hidden];
}

- (UIInterfaceOrientation)statusBarOrientation {
    env.window().orientation()
}

- (())sendEvent:(id)event {
    let windows = env.framework_state.uikit.ui_window.windows.clone();
    for window in windows {
        let _: () = msg![env; window sendEvent:event];
    }
}

- (id)windows {
    let all_windows = (&env.framework_state.uikit.ui_window.windows).to_vec();
    let array = ns_array::from_id_slice(env, &all_windows);
    autorelease(env, array)
}

- (id)keyWindow {
    env.framework_state.uikit.ui_window.key_window.unwrap_or(nil)
}

@end

};

#[no_mangle]
extern "C" fn UIApplicationMain(
    env: &mut Environment,
    _argc: i32,
    _argv: MutPtr<MutPtr<u8>>,
    _principal_class_name: id,
    delegate_class_name: id,
) -> i32 {
    let ui_application_class = msg_class![env; UIApplication class];
    let shared_application_alloc: id = msg![env; ui_application_class alloc];
    let shared_application: id = msg![env; shared_application_alloc init];

    env.framework_state.uikit.ui_application.shared_application = Some(shared_application);

    let delegate: id = if delegate_class_name != nil {
        // Извлекаем имя класса заранее
        let class_name_str = get_static_str(env, delegate_class_name);
        let delegate_class: id = env.objc.get_class(class_name_str);
        let delegate_alloc: id = msg![env; delegate_class alloc];
        msg![env; delegate_alloc init]
    } else {
        nil
    };

    if delegate != nil {
        let _: () = msg![env; shared_application setDelegate:delegate];
    }

    if let Some(main_nib_filename_rust) = env.bundle.main_nib_filename(None) {
        let main_nib_filename = from_rust_string(env, main_nib_filename_rust);
        let ns_bundle_class = msg_class![env; NSBundle class];
        let main_bundle: id = msg![env; ns_bundle_class mainBundle];
        let ok: bool = msg![env; main_bundle loadNibNamed:main_nib_filename owner:shared_application options:nil];
        if !ok {
            echo!("Warning: couldn't load main nib file");
        }
    }

    if delegate != nil {
        // В touchHLE проверка respondsToSelector и вызовы делаются так:
        let sel_old = env.objc.selector("applicationDidFinishLaunching:");
        let sel_new = env.objc.selector("application:didFinishLaunchingWithOptions:");

        if msg![env; delegate respondsToSelector:sel_old] {
            let _: () = msg![env; delegate applicationDidFinishLaunching:shared_application];
        } else if msg![env; delegate respondsToSelector:sel_new] {
            let _: () = msg![env; delegate application:shared_application didFinishLaunchingWithOptions:nil];
        }
    }

    let notification_center_class = msg_class![env; NSNotificationCenter class];
    let default_center: id = msg![env; notification_center_class defaultCenter];
    let notification_name = from_rust_string(env, UIApplicationDidFinishLaunchingNotification.to_string());
    let _: () = msg![env; default_center postNotificationName:notification_name object:shared_application];

    0
}

pub const UIApplicationDidFinishLaunchingNotification: &str = "UIApplicationDidFinishLaunchingNotification";
pub const UIApplicationDidBecomeActiveNotification: &str = "UIApplicationDidBecomeActiveNotification";
pub const UIApplicationDidEnterBackgroundNotification: &str = "UIApplicationDidEnterBackgroundNotification";
pub const UIApplicationWillEnterForegroundNotification: &str = "UIApplicationWillEnterForegroundNotification";
pub const UIApplicationWillResignActiveNotification: &str = "UIApplicationWillResignActiveNotification";
pub const UIApplicationWillTerminateNotification: &str = "UIApplicationWillTerminateNotification";
pub const UIApplicationDidReceiveMemoryWarningNotification: &str = "UIApplicationDidReceiveMemoryWarningNotification";
pub const UIApplicationLaunchOptionsRemoteNotificationKey: &str = "UIApplicationLaunchOptionsRemoteNotificationKey";

pub const CONSTANTS: ConstantExports = &[
    ("_UIApplicationDidFinishLaunchingNotification", HostConstant::NSString(UIApplicationDidFinishLaunchingNotification)),
    ("_UIApplicationDidBecomeActiveNotification", HostConstant::NSString(UIApplicationDidBecomeActiveNotification)),
    ("_UIApplicationDidEnterBackgroundNotification", HostConstant::NSString(UIApplicationDidEnterBackgroundNotification)),
    ("_UIApplicationWillEnterForegroundNotification", HostConstant::NSString(UIApplicationWillEnterForegroundNotification)),
    ("_UIApplicationWillResignActiveNotification", HostConstant::NSString(UIApplicationWillResignActiveNotification)),
    ("_UIApplicationWillTerminateNotification", HostConstant::NSString(UIApplicationWillTerminateNotification)),
    ("_UIApplicationDidReceiveMemoryWarningNotification", HostConstant::NSString(UIApplicationDidReceiveMemoryWarningNotification)),
    ("_UIApplicationLaunchOptionsRemoteNotificationKey", HostConstant::NSString(UIApplicationLaunchOptionsRemoteNotificationKey)),
];

pub const FUNCTIONS: FunctionExports = &[export_c_func!(UIApplicationMain(_, _, _, _))];

