/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `math.h`

use crate::dyld::{export_c_func, FunctionExports};
use crate::libc::errno::set_errno;
use crate::mem::MutPtr;
use crate::Environment;

// The sections in this file are organized to match the C standard.

// FIXME: Many functions in this file should theoretically set errno or affect
//        the floating-point environment. We're hoping apps won't rely on that.

fn abs(_env: &mut Environment, arg: i32) -> i32 {
    arg.abs()
}
fn fabs(_env: &mut Environment, arg: f64) -> f64 {
    arg.abs()
}

// Trigonometric functions

// TODO: These should also have `long double` variants, which can probably just
// alias the `double` ones.

fn sin(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.sin()
}
fn sinf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.sin()
}
fn cos(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.cos()
}
fn cosf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.cos()
}
fn tan(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.tan()
}
fn tanf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.tan()
}

fn asin(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.asin()
}
fn asinf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.asin()
}
fn acos(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.acos()
}
fn acosf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.acos()
}
fn atan(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.atan()
}
fn atanf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.atan()
}

fn atan2f(env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg1.atan2(arg2)
}
fn atan2(env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg1.atan2(arg2)
}

// Hyperbolic functions

fn sinh(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.sinh()
}
fn sinhf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.sinh()
}
fn cosh(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.cosh()
}
fn coshf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.cosh()
}
fn tanh(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.tanh()
}
fn tanhf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.tanh()
}

fn asinh(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.asinh()
}
fn asinhf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.asinh()
}
fn acosh(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.acosh()
}
fn acoshf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.acosh()
}
fn atanh(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.atanh()
}
fn atanhf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.atanh()
}

// Exponential and logarithmic functions
// TODO: implement the rest
fn log(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.ln()
}
fn logf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.ln()
}
fn log1p(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.ln_1p()
}
fn log1pf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.ln_1p()
}
fn log2(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.log2()
}
fn log2f(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.log2()
}
fn log10(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.log10()
}
fn log10f(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.log10()
}
fn exp(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.exp()
}
fn expf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.exp()
}
fn expm1(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.exp_m1()
}
fn expm1f(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.exp_m1()
}
fn exp2(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.exp2()
}
fn exp2f(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.exp2()
}
fn ldexp(env: &mut Environment, arg: f64, n: i32) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    assert!(!arg.is_infinite()); // TODO

    arg * 2f64.powf(n as _)
}
fn ldexpf(env: &mut Environment, arg: f32, n: i32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    assert!(!arg.is_infinite()); // TODO

    arg * 2f32.powf(n as _)
}
fn frexpf(env: &mut Environment, arg: f32, exp: MutPtr<i32>) -> f32 {
    frexp(env, arg.into(), exp) as f32
}
fn frexp(env: &mut Environment, arg: f64, exp: MutPtr<i32>) -> f64 {
    if arg == 0.0 {
        env.mem.write(exp, 0);
        return 0.0;
    }
    if arg < 0.0 {
        return -frexp(env, -arg, exp);
    }
    let b = arg.log2().floor() as i32 + 1;
    env.mem.write(exp, b);
    let frac = arg / 2f64.powi(b);
    assert!(
        (0.5..1.0).contains(&frac),
        "arg {}, b {}, frac {}",
        arg,
        b,
        frac
    );
    frac
}

// Power functions
// TODO: implement the rest
fn pow(env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg1.powf(arg2)
}
fn powf(env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg1.powf(arg2)
}
fn sqrt(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.sqrt()
}
fn sqrtf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.sqrt()
}

// Nearest integer functions
// TODO: implement the rest
fn ceil(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.ceil()
}
fn ceilf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.ceil()
}
fn floor(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.floor()
}
fn floorf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.floor()
}
fn round(env: &mut Environment, arg: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.round()
}
fn roundf(env: &mut Environment, arg: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg.round()
}
fn trunc(_env: &mut Environment, arg: f64) -> f64 {
    arg.trunc()
}
fn truncf(_env: &mut Environment, arg: f32) -> f32 {
    arg.trunc()
}
fn modff(env: &mut Environment, val: f32, iptr: MutPtr<f32>) -> f32 {
    let ivalue = truncf(env, val);
    env.mem.write(iptr, ivalue);
    val - ivalue
}
fn lrint(env: &mut Environment, arg: f64) -> i32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    // As tested on both macOS and iOS Simulator, by default it
    // rounds to the nearest integer with ties on even
    // TODO: support other rounding modes
    arg.max(i32::MIN as f64)
        .min(i32::MAX as f64)
        .round_ties_even() as i32
}
fn lrintf(env: &mut Environment, arg: f32) -> i32 {
    lrint(env, arg.into())
}

// Remainder functions
// TODO: implement the rest
fn fmod(env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg1 % arg2
}
fn fmodf(env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    // TODO: handle errno properly
    set_errno(env, 0);

    arg1 % arg2
}

// Maximum, minimum and positive difference functions
// TODO: implement fdim
fn fmax(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.max(arg2)
}
fn fmaxf(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.max(arg2)
}
fn fmin(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn fminf(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn poll(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_condattr_init(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_mutexattr_setpshared(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_yield_np(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCreateShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glShaderSource(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glSampleCoverage(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCompileShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetShaderiv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCreateProgram(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glAttachShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glLinkProgram(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetProgramiv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetAttribLocation(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetUniformLocation(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glPointSizePointerOES(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glDrawTexfOES(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glDeleteShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetFixedv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetTexEnvxv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glDrawTexiOES(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn abort(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioServicesCreateSystemSoundID(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioServicesDisposeSystemSoundID(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioQueueEnqueueBufferWithParameters(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioQueueSetProperty(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn asctime(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn backtrace_symbols(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn backtrace(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn class_getInstanceSize(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn compress2(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn creat(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn ctime(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn flockfile(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn funlockfile(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn __srget(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn fscanf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn getc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn ungetc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sranddev(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn srandomdev(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn strerror_r(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn strcasestr(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn task_get_exception_ports(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGColorEqualToColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGColorGetConstantColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextAddArcToPoint(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextAddLineToPoint(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextAddPath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextAddRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextBeginPath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextClip(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextClipToMask(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextClipToRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextClosePath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextDrawLayerInRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextDrawLinearGradient(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextDrawPath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextFillEllipseInRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextFillPath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextMoveToPoint(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSelectFont(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetTextDrawingMode(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetAllowsAntialiasing(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetAlpha(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetFillColorWithColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetFont(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetInterpolationQuality(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetLineCap(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetLineDash(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetLineJoin(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetLineWidth(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetRGBStrokeColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetShadow(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetShadowWithColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetShouldAntialias(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetStrokeColorWithColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetShouldSmoothFonts(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetTextPosition(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextStrokePath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextStrokeEllipseInRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGDataProviderCreateDirect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGDataProviderCreateSequential(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGDataProviderCreateWithCFData(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGDataProviderCreateWithFilename(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGDataProviderCreateWithURL(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGFontCreateWithFontName(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGFontCreateWithDataProvider(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGGradientCreateWithColorComponents(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreate(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreateCopy(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreateWithJPEGDataProvider(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreateWithImageInRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreateWithMask(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGLayerCreateWithContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGLayerGetContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGPathAddLines(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGPathAddRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGPathCloseSubpath(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGPathCreateCopy(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGPathCreateMutable(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGPathRelease(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGRectIntersection(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGRectGetHeight(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGRectIsNull(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGRectInset(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGRectIsEmpty(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGRectGetMidX(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn ExtAudioFileWrapAudioFileID(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn ExtAudioFileOpenURL(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn ExtAudioFileGetProperty(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn ExtAudioFileSetProperty(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn ExtAudioFileDispose(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn NSDefaultMallocZone(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn NSZoneMalloc(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIApplicationDidReceiveMemoryWarningNotification(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIGraphicsEndImageContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIGraphicsGetImageFromCurrentImageContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIImageWriteToSavedPhotosAlbum(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIRectFill(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIRectFrame(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_attr_setinheritsched(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_attr_setschedpolicy(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_get_stackaddr_np(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn pthread_get_stacksize_np(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn sqlite3_open(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_errcode(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_errmsg(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_prepare_v2(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_step(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_finalize(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_mprintf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_close(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_reset(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_int(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_double(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_parameter_count(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_get_table(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_free_table(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_exec(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_column_int(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_text(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_column_text(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_last_insert_rowid(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_prepare(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_column_count(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_column_name(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_parameter_index(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn lroundf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn lround(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn lrand48(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn rand_r(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn mbsrtowcs(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn mprotect(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn regcomp(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn setvbuf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn strftime(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn strerror(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn uncompress(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn wcstok(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn wcstod(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn writev(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CACurrentMediaTime(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CATransform3DMakeRotation(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CCCrypt(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CC_MD5_Final(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CC_MD5_Init(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CC_MD5_Update(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CC_SHA1(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CC_SHA1_Init(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CC_SHA1_Update(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFAbsoluteTimeGetDifferenceAsGregorianUnits(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFAllocatorGetDefault(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringCreateWithBytes(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringCreateWithFileSystemRepresentation(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringGetBytes(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringGetCharacterAtIndex(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringGetSystemEncoding(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringHasSuffix(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringCreateMutableCopy(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringNormalize(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringCreateWithCharactersNoCopy(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFNetServiceBrowserCreate(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopAddSource(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopRun(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopSourceCreate(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopStop(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopContainsTimer(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopRemoveTimer(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGColorGetAlpha(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFBundleCopyLocalizedString(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFBundleCopyResourceURLForLocalization(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFBundleGetInfoDictionary(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFBundleCopyExecutableURL(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFURLCreateDataAndPropertiesFromResource(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFURLCreateWithString(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFURLCreateStringByAddingPercentEscapes(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn MFMailComposeErrorDomain(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn lstat(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn clock_get_time(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn connect(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn difftime(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn div(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn SCNetworkReachabilityScheduleWithRunLoop(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn OSAtomicCompareAndSwapInt(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn UIGraphicsBeginImageContext(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn mach_thread_self(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn dladdr(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sched_get_priority_max(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn pthread_key_delete(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlNewParserCtxt(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlCtxtReadMemory(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlDocGetRootElement(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlClearParserCtxt(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlFreeParserCtxt(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn gethostent(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn objc_getClass(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn __memcpy_chk(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlCleanupParser(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glLogicOp(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn wcsftime(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn pthread_exit(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn ExtAudioFileRead(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn AudioServicesAddSystemSoundCompletion(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn mstats(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFStringGetCharactersPtr(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn localeconv(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFURLCreateWithBytes(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFHostCreateWithName(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGGradientRelease(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn wcstol(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn fgetwc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFHostStartInfoResolution(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn vasprintf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGPathMoveToPoint(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGPathAddPath(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFHostGetAddressing(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGPathAddLineToPoint(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGContextSetTextMatrix(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGContextShowTextAtPoint(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGPathContainsPoint(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sysconf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn _NSGetExecutablePath(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CGImageGetBitmapInfo(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn __CFStringMakeConstantString(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn SCNetworkReachabilityUnscheduleFromRunLoop(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFUUIDCreate(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn freopen(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFUUIDCreateString(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_column_int64(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlReadFile(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlFreeDoc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn xmlGetProp(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn OSSpinLockLock(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn OSMemoryBarrier(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn valloc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn OSSpinLockUnlock(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn CFStreamCreatePairWithSocketToHost(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn objc_getMetaClass(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn class_getInstanceMethod(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn method_getTypeEncoding(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn class_replaceMethod(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn memset_pattern16(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glGenFramebuffers(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glBindFramebuffer(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glGenRenderbuffers(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glBindRenderbuffer(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn umask(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glGetRenderbufferParameteriv(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glFramebufferRenderbuffer(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glCheckFramebufferStatus(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn glDiscardFramebufferEXT(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(abs(_)),
    export_c_func!(fabs(_)),
    // Trigonometric functions
    export_c_func!(sin(_)),
    export_c_func!(sinf(_)),
    export_c_func!(cos(_)),
    export_c_func!(cosf(_)),
    export_c_func!(tan(_)),
    export_c_func!(tanf(_)),
    export_c_func!(asin(_)),
    export_c_func!(asinf(_)),
    export_c_func!(acos(_)),
    export_c_func!(acosf(_)),
    export_c_func!(atan(_)),
    export_c_func!(atanf(_)),
    export_c_func!(atan2(_, _)),
    export_c_func!(atan2f(_, _)),
    // Hyperbolic functions
    export_c_func!(sinh(_)),
    export_c_func!(sinhf(_)),
    export_c_func!(cosh(_)),
    export_c_func!(coshf(_)),
    export_c_func!(tanh(_)),
    export_c_func!(tanhf(_)),
    export_c_func!(asinh(_)),
    export_c_func!(asinhf(_)),
    export_c_func!(acosh(_)),
    export_c_func!(acoshf(_)),
    export_c_func!(atanh(_)),
    export_c_func!(atanhf(_)),
    // Exponential and logarithmic functions
    export_c_func!(log(_)),
    export_c_func!(logf(_)),
    export_c_func!(log1p(_)),
    export_c_func!(log1pf(_)),
    export_c_func!(log2(_)),
    export_c_func!(log2f(_)),
    export_c_func!(log10(_)),
    export_c_func!(log10f(_)),
    export_c_func!(exp(_)),
    export_c_func!(expf(_)),
    export_c_func!(expm1(_)),
    export_c_func!(expm1f(_)),
    export_c_func!(exp2(_)),
    export_c_func!(exp2f(_)),
    export_c_func!(ldexp(_, _)),
    export_c_func!(ldexpf(_, _)),
    export_c_func!(frexpf(_, _)),
    export_c_func!(frexp(_, _)),
    // Power functions
    export_c_func!(pow(_, _)),
    export_c_func!(powf(_, _)),
    export_c_func!(sqrt(_)),
    export_c_func!(sqrtf(_)),
    // Nearest integer functions
    export_c_func!(ceil(_)),
    export_c_func!(ceilf(_)),
    export_c_func!(floor(_)),
    export_c_func!(floorf(_)),
    export_c_func!(round(_)),
    export_c_func!(roundf(_)),
    export_c_func!(trunc(_)),
    export_c_func!(truncf(_)),
    export_c_func!(modff(_, _)),
    export_c_func!(lrint(_)),
    export_c_func!(lrintf(_)),
    // Remainder functions
    export_c_func!(fmod(_, _)),
    export_c_func!(fmodf(_, _)),
    // Maximum, minimum and positive difference functions
    export_c_func!(fmax(_, _)),
    export_c_func!(fmaxf(_, _)),
    export_c_func!(fmin(_, _)),
    export_c_func!(fminf(_, _)),
    export_c_func!(poll(_, _)),
    export_c_func!(pthread_condattr_init(_, _)),
    export_c_func!(pthread_mutexattr_setpshared(_, _)),
    export_c_func!(pthread_yield_np(_, _)),
    export_c_func!(glCreateShader(_, _)),
    export_c_func!(glShaderSource(_, _)),
    export_c_func!(glSampleCoverage(_, _)),
    export_c_func!(glCompileShader(_, _)),
    export_c_func!(glGetShaderiv(_, _)),
    export_c_func!(glCreateProgram(_, _)),
    export_c_func!(glAttachShader(_, _)),
    export_c_func!(glLinkProgram(_, _)),
    export_c_func!(glGetProgramiv(_, _)),
    export_c_func!(glGetAttribLocation(_, _)),
    export_c_func!(glGetUniformLocation(_, _)),
    export_c_func!(glDrawTexfOES(_, _)),
    export_c_func!(glDeleteShader(_, _)),
    export_c_func!(glGetFixedv(_, _)),
    export_c_func!(glPointSizePointerOES(_, _)),
    export_c_func!(glGetTexEnvxv(_, _)),
    export_c_func!(glDrawTexiOES(_, _)),
    export_c_func!(abort(_, _)),
    export_c_func!(AudioServicesCreateSystemSoundID(_, _)),
    export_c_func!(AudioServicesDisposeSystemSoundID(_, _)),
    export_c_func!(AudioQueueEnqueueBufferWithParameters(_, _)),
    export_c_func!(AudioQueueSetProperty(_, _)),
    export_c_func!(asctime(_, _)),
    export_c_func!(backtrace_symbols(_, _)),
    export_c_func!(backtrace(_, _)),
    export_c_func!(class_getInstanceSize(_, _)),
    export_c_func!(compress2(_, _)),
    export_c_func!(creat(_, _)),
    export_c_func!(ctime(_, _)),
    export_c_func!(flockfile(_, _)),
    export_c_func!(funlockfile(_, _)),
    export_c_func!(__srget(_, _)),
    export_c_func!(fscanf(_, _)),
    export_c_func!(getc(_, _)),
    export_c_func!(ungetc(_, _)),
    export_c_func!(CGColorEqualToColor(_, _)),
    export_c_func!(CGColorGetConstantColor(_, _)),
    export_c_func!(CGContextAddArcToPoint(_, _)),
    export_c_func!(CGContextAddLineToPoint(_, _)),
    export_c_func!(CGContextAddPath(_, _)),
    export_c_func!(CGContextAddRect(_, _)),
    export_c_func!(CGContextBeginPath(_, _)),
    export_c_func!(CGContextClip(_, _)),
    export_c_func!(CGContextClipToMask(_, _)),
    export_c_func!(CGContextClipToRect(_, _)),
    export_c_func!(CGContextClosePath(_, _)),
    export_c_func!(CGContextDrawLayerInRect(_, _)),
    export_c_func!(CGContextDrawLinearGradient(_, _)),
    export_c_func!(CGContextDrawPath(_, _)),
    export_c_func!(CGContextFillEllipseInRect(_, _)),
    export_c_func!(CGContextFillPath(_, _)),
    export_c_func!(CGContextMoveToPoint(_, _)),
    export_c_func!(CGContextSelectFont(_, _)),
    export_c_func!(CGContextSetTextDrawingMode(_, _)),
    export_c_func!(CGContextSetAllowsAntialiasing(_, _)),
    export_c_func!(CGContextSetAlpha(_, _)),
    export_c_func!(CGContextSetFillColorWithColor(_, _)),
    export_c_func!(CGContextSetFont(_, _)),
    export_c_func!(CGContextSetInterpolationQuality(_, _)),
    export_c_func!(CGContextSetLineCap(_, _)),
    export_c_func!(CGContextSetLineDash(_, _)),
    export_c_func!(CGContextSetLineJoin(_, _)),
    export_c_func!(CGContextSetLineWidth(_, _)),
    export_c_func!(CGContextSetRGBStrokeColor(_, _)),
    export_c_func!(CGContextSetShadow(_, _)),
    export_c_func!(CGContextSetShadowWithColor(_, _)),
    export_c_func!(CGContextSetShouldAntialias(_, _)),
    export_c_func!(CGContextSetStrokeColorWithColor(_, _)),
    export_c_func!(CGContextSetShouldSmoothFonts(_, _)),
    export_c_func!(CGContextSetTextPosition(_, _)),
    export_c_func!(CGContextStrokePath(_, _)),
    export_c_func!(CGContextStrokeEllipseInRect(_, _)),
    export_c_func!(CGDataProviderCreateDirect(_, _)),
    export_c_func!(CGDataProviderCreateSequential(_, _)),
    export_c_func!(CGDataProviderCreateWithCFData(_, _)),
    export_c_func!(CGDataProviderCreateWithURL(_, _)),
    export_c_func!(CGDataProviderCreateWithFilename(_, _)),
    export_c_func!(CGFontCreateWithFontName(_, _)),
    export_c_func!(CGFontCreateWithDataProvider(_, _)),
    export_c_func!(CGGradientCreateWithColorComponents(_, _)),
    export_c_func!(CGImageCreate(_, _)),
    export_c_func!(CGImageCreateCopy(_, _)),
    export_c_func!(CGImageCreateWithJPEGDataProvider(_, _)),
    export_c_func!(CGImageCreateWithImageInRect(_, _)),
    export_c_func!(CGImageCreateWithMask(_, _)),
    export_c_func!(CGLayerCreateWithContext(_, _)),
    export_c_func!(CGLayerGetContext(_, _)),
    export_c_func!(CGPathAddLines(_, _)),
    export_c_func!(CGPathAddRect(_, _)),
    export_c_func!(CGPathCloseSubpath(_, _)),
    export_c_func!(CGPathCreateCopy(_, _)),
    export_c_func!(CGPathCreateMutable(_, _)),
    export_c_func!(CGPathRelease(_, _)),
    export_c_func!(CGRectIntersection(_, _)),
    export_c_func!(CGRectGetHeight(_, _)),
    export_c_func!(CGRectIsNull(_, _)),
    export_c_func!(CGRectInset(_, _)),
    export_c_func!(CGRectIsEmpty(_, _)),
    export_c_func!(CGRectGetMidX(_, _)),
    export_c_func!(ExtAudioFileWrapAudioFileID(_, _)),
    export_c_func!(ExtAudioFileOpenURL(_, _)),
    export_c_func!(ExtAudioFileGetProperty(_, _)),
    export_c_func!(ExtAudioFileSetProperty(_, _)),
    export_c_func!(ExtAudioFileDispose(_, _)),
    export_c_func!(NSDefaultMallocZone(_, _)),
    export_c_func!(NSZoneMalloc(_, _)),
    export_c_func!(UIApplicationDidReceiveMemoryWarningNotification(_, _)),
    export_c_func!(UIGraphicsEndImageContext(_, _)),
    export_c_func!(UIGraphicsGetImageFromCurrentImageContext(_, _)),
    export_c_func!(UIImageWriteToSavedPhotosAlbum(_, _)),
    export_c_func!(UIRectFill(_, _)),
    export_c_func!(UIRectFrame(_, _)),
    export_c_func!(pthread_attr_setinheritsched(_, _)),
    export_c_func!(pthread_attr_setschedpolicy(_, _)),
    export_c_func!(pthread_get_stacksize_np(_, _)),
    export_c_func!(pthread_get_stackaddr_np(_, _)),
    export_c_func!(sqlite3_open(_, _)),
    export_c_func!(sqlite3_errcode(_, _)),
    export_c_func!(sqlite3_errmsg(_, _)),
    export_c_func!(sqlite3_prepare_v2(_, _)),
    export_c_func!(sqlite3_step(_, _)),
    export_c_func!(sqlite3_finalize(_, _)),
    export_c_func!(sqlite3_mprintf(_, _)),
    export_c_func!(sqlite3_close(_, _)),
    export_c_func!(sqlite3_reset(_, _)),
    export_c_func!(sqlite3_bind_int(_, _)),
    export_c_func!(sqlite3_bind_double(_, _)),
    export_c_func!(sqlite3_bind_parameter_count(_, _)),
    export_c_func!(sqlite3_get_table(_, _)),
    export_c_func!(sqlite3_free_table(_, _)),
    export_c_func!(sqlite3_exec(_, _)),
    export_c_func!(sqlite3_column_int(_, _)),
    export_c_func!(sqlite3_bind_text(_, _)),
    export_c_func!(sqlite3_column_text(_, _)),
    export_c_func!(sqlite3_last_insert_rowid(_, _)),
    export_c_func!(sqlite3_prepare(_, _)),
    export_c_func!(sqlite3_column_count(_, _)),
    export_c_func!(sqlite3_column_name(_, _)),
    export_c_func!(sqlite3_bind_parameter_index(_, _)),
    export_c_func!(lroundf(_, _)),
    export_c_func!(lround(_, _)),
    export_c_func!(lrand48(_, _)),
    export_c_func!(rand_r(_, _)),
    export_c_func!(mbsrtowcs(_, _)),
    export_c_func!(mprotect(_, _)),
    export_c_func!(regcomp(_, _)),
    export_c_func!(setvbuf(_, _)),
    export_c_func!(strftime(_, _)),
    export_c_func!(strerror(_, _)),
    export_c_func!(uncompress(_, _)),
    export_c_func!(wcstok(_, _)),
    export_c_func!(wcstod(_, _)),
    export_c_func!(writev(_, _)),
    export_c_func!(CACurrentMediaTime(_, _)),
    export_c_func!(CATransform3DMakeRotation(_, _)),
    export_c_func!(CCCrypt(_, _)),
    export_c_func!(CC_MD5_Final(_, _)),
    export_c_func!(CC_MD5_Init(_, _)),
    export_c_func!(CC_MD5_Update(_, _)),
    export_c_func!(CC_SHA1(_, _)),
    export_c_func!(CC_SHA1_Init(_, _)),
    export_c_func!(CC_SHA1_Update(_, _)),
    export_c_func!(CFAbsoluteTimeGetDifferenceAsGregorianUnits(_, _)),
    export_c_func!(CFAllocatorGetDefault(_, _)),
    export_c_func!(CFStringCreateWithBytes(_, _)),
    export_c_func!(CFStringCreateWithFileSystemRepresentation(_, _)),
    export_c_func!(CFStringGetBytes(_, _)),
    export_c_func!(CFStringGetCharacterAtIndex(_, _)),
    export_c_func!(CFStringGetSystemEncoding(_, _)),
    export_c_func!(CFStringHasSuffix(_, _)),
    export_c_func!(CFStringCreateMutableCopy(_, _)),
    export_c_func!(CFStringNormalize(_, _)),
    export_c_func!(CFStringCreateWithCharactersNoCopy(_, _)),
    export_c_func!(CFNetServiceBrowserCreate(_, _)),
    export_c_func!(CFRunLoopAddSource(_, _)),
    export_c_func!(CFRunLoopRun(_, _)),
    export_c_func!(CFRunLoopSourceCreate(_, _)),
    export_c_func!(CFRunLoopStop(_, _)),
    export_c_func!(CFRunLoopContainsTimer(_, _)),
    export_c_func!(CFRunLoopRemoveTimer(_, _)),
    export_c_func!(CGColorGetAlpha(_, _)),
    export_c_func!(CFBundleCopyLocalizedString(_, _)),
    export_c_func!(CFBundleCopyResourceURLForLocalization(_, _)),
    export_c_func!(CFBundleGetInfoDictionary(_, _)),
    export_c_func!(CFBundleCopyExecutableURL(_, _)),
    export_c_func!(CFURLCreateDataAndPropertiesFromResource(_, _)),
    export_c_func!(CFURLCreateWithString(_, _)),
    export_c_func!(CFURLCreateStringByAddingPercentEscapes(_, _)),
    export_c_func!(MFMailComposeErrorDomain(_, _)),
    export_c_func!(lstat(_, _)),
    export_c_func!(clock_get_time(_, _)),
    export_c_func!(connect(_, _)),
    export_c_func!(difftime(_, _)),
    export_c_func!(div(_, _)),
    export_c_func!(SCNetworkReachabilityScheduleWithRunLoop(_, _)),
    export_c_func!(OSAtomicCompareAndSwapInt(_, _)),
    export_c_func!(UIGraphicsBeginImageContext(_, _)),
    export_c_func!(mach_thread_self(_, _)),
    export_c_func!(dladdr(_, _)),
    export_c_func!(sched_get_priority_max(_, _)),
    export_c_func!(pthread_key_delete(_, _)),
    export_c_func!(xmlNewParserCtxt(_, _)),
    export_c_func!(xmlCtxtReadMemory(_, _)),
    export_c_func!(xmlDocGetRootElement(_, _)),
    export_c_func!(xmlClearParserCtxt(_, _)),
    export_c_func!(xmlFreeParserCtxt(_, _)),
    export_c_func!(gethostent(_, _)),
    export_c_func!(objc_getClass(_, _)),
    export_c_func!(__memcpy_chk(_, _)),
    export_c_func!(xmlCleanupParser(_, _)),
    export_c_func!(glLogicOp(_, _)),
    export_c_func!(wcsftime(_, _)),
    export_c_func!(pthread_exit(_, _)),
    export_c_func!(ExtAudioFileRead(_, _)),
    export_c_func!(AudioServicesAddSystemSoundCompletion(_, _)),
    export_c_func!(mstats(_, _)),
    export_c_func!(srandomdev(_, _)),
    export_c_func!(strcasestr(_, _)),
    export_c_func!(CFStringGetCharactersPtr(_, _)),
    export_c_func!(localeconv(_, _)),
    export_c_func!(CFURLCreateWithBytes(_, _)),
    export_c_func!(CFHostCreateWithName(_, _)),
    export_c_func!(CGGradientRelease(_, _)),
    export_c_func!(wcstol(_, _)),
    export_c_func!(sranddev(_, _)),
    export_c_func!(fgetwc(_, _)),
    export_c_func!(CFHostStartInfoResolution(_, _)),
    export_c_func!(vasprintf(_, _)),
    export_c_func!(CGPathMoveToPoint(_, _)),
    export_c_func!(CGPathAddPath(_, _)),
    export_c_func!(CFHostGetAddressing(_, _)),
    export_c_func!(CGPathAddLineToPoint(_, _)),
    export_c_func!(CGContextSetTextMatrix(_, _)),
    export_c_func!(CGContextShowTextAtPoint(_, _)),
    export_c_func!(CGPathContainsPoint(_, _)),
    export_c_func!(sysconf(_, _)),
    export_c_func!(_NSGetExecutablePath(_, _)),
    export_c_func!(CGImageGetBitmapInfo(_, _)),
    export_c_func!(__CFStringMakeConstantString(_, _)),
    export_c_func!(SCNetworkReachabilityUnscheduleFromRunLoop(_, _)),
    export_c_func!(CFUUIDCreate(_, _)),
    export_c_func!(freopen(_, _)),
    export_c_func!(CFUUIDCreateString(_, _)),
    export_c_func!(sqlite3_column_int64(_, _)),
    export_c_func!(xmlReadFile(_, _)),
    export_c_func!(xmlFreeDoc(_, _)),
    export_c_func!(xmlGetProp(_, _)),
    export_c_func!(OSSpinLockLock(_, _)),
    export_c_func!(OSMemoryBarrier(_, _)),
    export_c_func!(valloc(_, _)),
    export_c_func!(OSSpinLockUnlock(_, _)),
    export_c_func!(CFStreamCreatePairWithSocketToHost(_, _)),
    export_c_func!(objc_getMetaClass(_, _)),
    export_c_func!(class_getInstanceMethod(_, _)),
    export_c_func!(method_getTypeEncoding(_, _)),
    export_c_func!(class_replaceMethod(_, _)),
    export_c_func!(memset_pattern16(_, _)),
    export_c_func!(glGenFramebuffers(_, _)),
    export_c_func!(glBindFramebuffer(_, _)),
    export_c_func!(glGenRenderbuffers(_, _)),
    export_c_func!(glBindRenderbuffer(_, _)),
    export_c_func!(umask(_, _)),
    export_c_func!(glGetRenderbufferParameteriv(_, _)),
    export_c_func!(glFramebufferRenderbuffer(_, _)),
    export_c_func!(glCheckFramebufferStatus(_, _)),
    export_c_func!(glDiscardFramebufferEXT(_, _)),
];
