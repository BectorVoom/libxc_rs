//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2632/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2632<F: Float>(t3447: F, t4904: F, t64779: F, t15402: F, t21749: F, t15376: F, t15382: F, t15390: F, t15395: F, t18543: F, t18546: F, t44635: F, t458: F, t4900: F, t4919: F, t4936: F, t52100: F, t52368: F, t6138: F, t64644: F, t64870: F, t65018: F, t65056: F, t71193: F, t72688: F) -> (F, F, F) {
    let t73535 = t3447 * t64779 * t4904;
    let t73541 = t3447 * t15402 * t21749;
    let t73571 = -F::cast_from(0.24999999999999999999e-2_f64) * t3447 * t458 * t6138 * t4936 + F::cast_from(0.66666666666666666663e-2_f64) * t3447 * t4900 * t71193 + t52368 - F::cast_from(0.25925925925925925926e-2_f64) * t3447 * t15395 * t72688 + F::cast_from(0.25925925925925925926e-2_f64) * t3447 * t52100 * t65018 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t64644 * t15382 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t15390 * t65056 - F::cast_from(0.44444444444444444443e-2_f64) * t15376 * t18543 - F::cast_from(0.88888888888888888886e-2_f64) * t15376 * t18546 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t4919 * t64870 - F::cast_from(0.10288065843621399177e-3_f64) * t44635;
    (t73535, t73541, t73571)
}
