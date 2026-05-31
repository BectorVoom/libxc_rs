//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2312/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2312<F: Float>(t1920: F, t28630: F, t968: F, t5872: F, t6768: F, t83244: F, t89242: F, t1058: F, t1060: F, t18047: F, t1948: F, t23346: F, t28663: F, t3200: F, t3201: F, t345: F, t4649: F, t5838: F, t6687: F, t6805: F, t7593: F, t89360: F, t89362: F, t89366: F, t89369: F, t89505: F, t986: F) -> (F, F, F) {
    let t100324 = t1920 * t968 * t28630;
    let t100326 = t6768 * t5872;
    let t100334 = t83244 * t89242;
    let t100341 = -t89360 - t89362 - F::cast_from(0.12184696791468343974e-2_f64) * t89366 + t89369 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t345 * t1948 * t18047 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t5838 * t6805 + F::cast_from(0.27415567780803773942e-2_f64) * t100324 - t3200 * t100326 * t3201 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t986 * t28630 + F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t28663 + F::cast_from(0.16449340668482264365e-1_f64) * t100334 * t89505 + F::cast_from(2.0_f64) * t1058 * t7593 * t4649 * t1060;
    (t100326, t100334, t100341)
}
