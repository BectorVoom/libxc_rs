//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1377/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1377<F: Float>(t225: F, t24871: F, t2122: F, t7319: F, t24574: F, t24597: F, t1235: F, t461: F, t24626: F, t24617: F, t11498: F, t1190: F, t11925: F, t1252: F, t2123: F, t24567: F, t24582: F, t24589: F, t24590: F, t24596: F, t24604: F, t24616: F, t24757: F, t24877: F, t27549: F, t3487: F, t3593: F, t498: F, t7283: F, t7356: F) -> F {
    let t86400 = t24871 * t225;
    let t86403 = t7319 * t2122;
    let t86409 = t24574 * t24597;
    let t86415 = t461 * t1235 * t225;
    let t86424 = t24574 * t24626;
    let t86426 = t24574 * t24617;
    let t86436 = -F::cast_from(3.0_f64) * t86400 * t1252 - F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t86403 * t24604 + F::cast_from(12.0_f64) * t3593 * t24582 + F::cast_from(0.36554090374405031922e-2_f64) * t86409 + F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t24567 * t24616 + F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t86415 * t24604 - F::cast_from(0.10966227112321509577e-1_f64) * t27549 * t24590 * t24596 + F::cast_from(6.0_f64) * t11925 * t7356 - F::cast_from(0.82246703342411321826e-2_f64) * t86424 + F::cast_from(0.16449340668482264365e-1_f64) * t86426 + F::cast_from(6.0_f64) * t3487 * t24877 + F::cast_from(3.0_f64) * t1190 * t24757 * t498 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t11498 * t2123;
    t86436
}
