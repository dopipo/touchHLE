/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIView`.

use crate::frameworks::core_graphics::{CGRect, cg_color};
use crate::objc::{id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject};
use crate::frameworks::quartz_core::ca_layer::CALayerHostObject;
use crate::Environment;

pub struct UIViewHostObject {
    pub layer: id,
    pub subviews: Vec<id>,
    pub superview: id,
}

impl HostObject for UIViewHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIView: UIResponder

- (id)initWithFrame:(CGRect)frame {
    this = msg![env; this init];
    if this != nil {
        // Создаем базовый слой CALayer для вью
        let layer_class = msg_class![env; CALayer class];
        let layer: id = msg![env; layer_class alloc];
        let layer: id = msg![env; layer init];
        
        let mut host_obj = Box::new(UIViewHostObject {
            layer,
            subviews: Vec::new(),
            superview: nil,
        });

        env.objc.set_host_object(this, host_obj);
        
        // Синхронизируем начальный фрейм со слоем
        msg![env; this setFrame:frame];
    }
    this
}

- (id)layer {
    env.objc.borrow::<UIViewHostObject>(this).layer
}

- (CGRect)frame {
    let layer = msg![env; this layer];
    msg![env; layer frame]
}

- ((()))setFrame:(CGRect)frame {
    let layer = msg![env; this layer];
    // Важно: изменение фрейма вью должно менять фрейм слоя
    msg![env; layer setFrame:frame];
}

- (CGRect)bounds {
    let layer = msg![env; this layer];
    msg![env; layer bounds]
}

- ((()))setBounds:(CGRect)bounds {
    let layer = msg![env; this layer];
    msg![env; layer setBounds:bounds];
}

- ((()))setBackgroundColor:(id)color {
    let layer = msg![env; this layer];
    // Извлечение CGColor из UIColor и передача слою
    let cg_color = if color != nil { msg![env; color CGColor] } else { nil };
    msg![env; layer setBackgroundColor:cg_color];
}

- ((()))setAlpha:(f32)alpha {
    let layer = msg![env; this layer];
    msg![env; layer setOpacity:alpha];
}

- ((()))setHidden:(bool)hidden {
    let layer = msg![env; this layer];
    msg![env; layer setHidden:hidden];
}

- ((()))addSubview:(id)view {
    if view == nil { return; }
    retain(env, view);
    
    let layer = msg![env; this layer];
    let subview_layer = msg![env; view layer];
    
    // Добавляем слой сабвью в иерархию слоев
    msg![env; layer addSublayer:subview_layer];
    
    let host_obj = env.objc.borrow_mut::<UIViewHostObject>(this);
    host_obj.subviews.push(view);
}

- ((()))removeFromSuperview {
    let superview = msg![env; this superview];
    if superview != nil {
        let layer = msg![env; this layer];
        msg![env; layer removeFromSuperlayer];
        
        // Логика удаления из вектора subviews родителя...
        // (упрощено для примера)
    }
}

@end

};
