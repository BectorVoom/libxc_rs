//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2335/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2335<F: Float>(t29787: F, t85639: F, t1170: F, t2121: F, t29726: F, t103337: F, t104453: F, t1244: F, t1246: F, t15027: F, t1716: F, t19201: F, t2147: F, t27454: F, t27471: F, t27507: F, t27511: F, t27543: F, t27725: F, t470: F, t491: F, t4928: F, t493: F, t5064: F, t6218: F, t7283: F, t7348: F, t7387: F, t95768: F, t95774: F) -> F {
    let t104469 = t85639 * t29787;
    let t104480 = t2121 * t1170 * t29726;
    let t104482 = t470 * t493 * t104453 + F::cast_from(0.97477574331746751793e-2_f64) * t95768 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t2147 * t491 * t4928 + F::new(2.0) * t5064 * t27471 + t1244 * t7348 * t6218 * t1246 + t95774 - F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t27511 + F::cast_from(0.18277045187202515961e-2_f64) * t104469 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t103337 * t27454 + F::new(2.0) * t5064 * t27725 + t19201 * t7387 + F::new(4.0) * t15027 * t27543 + F::cast_from(0.27415567780803773942e-2_f64) * t104480;
    t104482
}
