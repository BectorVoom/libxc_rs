//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 847/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk847<F: Float>(t221: F, t5196: F, t6347: F, t12188: F, t12194: F, t12196: F, t12215: F, t12236: F, t1315: F, t16078: F, t16108: F, t16119: F, t19768: F, t19776: F, t19779: F, t19791: F, t20576: F, t20582: F, t5195: F) -> F {
    let t20586 = t221 * t5196 * t6347;
    let t20594 = -F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t20576 - t12188 - F::cast_from(0.74999999999999999997e-2_f64) * t19768 + F::cast_from(0.24999999999999999999e-2_f64) * t19776 - t12194 + t12196 - F::cast_from(0.19999999999999999999e-1_f64) * t12215 * t20582 + F::cast_from(0.14999999999999999999e-1_f64) * t5195 * t20586 - F::cast_from(0.34999999999999999998e-1_f64) * t19779 + F::cast_from(0.11666666666666666666e-1_f64) * t19791 - F::cast_from(0.38888888888888888888e-1_f64) * t16078 - t12236 - F::cast_from(0.15833333333333333333e-1_f64) * t16108 + F::cast_from(0.49999999999999999998e-2_f64) * t16119;
    t20594
}
