/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CAEAGLLayer`.

use super::ca_layer::CALayerHostObject;
use crate::frameworks::core_graphics::{CGPoint, CGRect};
use crate::objc::{id, msg, msg_class, nil, objc_classes, Class, ClassExports};
use crate::Environment;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation CAEAGLLayer: CALayer

// EAGLDrawable implementation (the only one)

- (id)drawableProperties {
    // FIXME: do we need to return an empty dictionary rather than nil?
    env.objc.borrow::<CALayerHostObject>(this).drawable_properties
}

- (id)main {
    nil
}

- (id)description {
    nil
}

- (())setDrawableProperties:(id)props { // NSDictionary<NSString*, id>*
    let props: id = msg![env; props copy];
    env.objc.borrow_mut::<CALayerHostObject>(this).drawable_properties = props;
}

- (())setContentsScale:(bool)scale {
    log!("TODO: setContentsScale:{}", scale);
}

@end

};

/// Если существует непрозрачный `CAEAGLLayer`, который занимает весь экран,
/// функция возвращает указатель на него. В противном случае — [nil].
pub fn find_fullscreen_eagl_layer(env: &mut Environment) -> id {
    let ui_window_class = msg_class![env; UIWindow class];
    let mut layer = nil;

    // Ищем основное окно
    for window in &env.framework_state.uikit.ui_window.windows {
        if !msg![env; *window isKindOfClass:ui_window_class] {
            continue;
        }
        layer = msg![env; *window layer];
        break;
    }

    if layer == nil {
        return nil;
    }

    // Спускаемся по иерархии слоев до самого глубокого
    loop {
        let layer_host_obj = env.objc.borrow::<CALayerHostObject>(layer);
        
        // Проверяем, что слой не трансформирован (должен быть identity)
        if !layer_host_obj.affine_transform.is_identity() {
            return nil;
        }

        if let Some(&next) = layer_host_obj.sublayers.last() {
            layer = next;
        } else {
            break;
        }
    }

    // Проверяем непрозрачность
    if !env.objc.borrow::<CALayerHostObject>(layer).opaque {
        return nil;
    }

    // Убеждаемся, что это именно CAEAGLLayer
    let ca_eagl_layer_class: Class = msg_class![env; CAEAGLLayer class];
    if !msg![env; layer isKindOfClass:ca_eagl_layer_class] {
        return nil;
    }

    layer
}

/// Используется `EAGLContext` для получения буфера пикселей перед отрисовкой.
pub fn get_pixels_vec_for_presenting(env: &mut Environment, layer: id) -> Vec<u8> {
    env.objc
        .borrow_mut::<CALayerHostObject>(layer)
        .presented_pixels
        .take()
        .map(|(vec, _width, _height)| vec)
        .unwrap_or_default()
}

/// Используется `EAGLContext` для передачи отрендеренного кадра слою.
pub fn present_pixels(env: &mut Environment, layer: id, pixels: Vec<u8>, width: u32, height: u32) {
    env.objc
        .borrow_mut::<CALayerHostObject>(layer)
        .presented_pixels = Some((pixels, width, height));
}
