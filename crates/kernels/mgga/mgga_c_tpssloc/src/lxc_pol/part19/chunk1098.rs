//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1098/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098<F: Float>(t3736: F, t40018: F, t12012: F, t12220: F, t16101: F, t210: F, t213: F, t214: F, t221: F, t3719: F, t3733: F, t3734: F, t39622: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t40360: F, t40366: F, t40372: F, t40376: F, t5195: F) -> (F,) {
    let t40387 = t40018 * t3736;
    let t40389 = -t40343 + t40347 + t40350 - 0.79999999999999999997e-1 * t40351 - 0.29999999999999999998e-1 * t40356 + 0.99999999999999999996e-2 * t40360 + 0.19999999999999999999e-1 * t5195 * t221 * t12220 * t12012 - 0.13999999999999999999e0 * t40366 + 0.11111111111111111111e-2 * t40372 - 0.29999999999999999998e-1 * t40376 - 0.11999999999999999999e0 * t16101 * t221 * t213 * t3734 * t3719 + 0.14999999999999999999e-1 * t3733 * t210 * t214 * t39622 + 0.23333333333333333332e0 * t40387;
    (t40389,)
}
