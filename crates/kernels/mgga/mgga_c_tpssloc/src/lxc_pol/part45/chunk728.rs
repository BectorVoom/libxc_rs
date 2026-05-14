//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 728/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk728<F: Float>(t1052: F, t1923: F, t23310: F, t23314: F, t23317: F, t23323: F, t23327: F, t23333: F, t23337: F, t23341: F, t23346: F, t23381: F, t23574: F, t23732: F, t3026: F, t3169: F, t6687: F, t6707: F, t6776: F) -> (F,) {
    let t23734 = -0.16449340668482264365e-1 * t6687 * t23310 - 0.82246703342411321825e-2 * t6687 * t23314 - 0.82246703342411321825e-2 * t6687 * t23317 + 4.0 * t3026 * t6776 + 0.80418998823691070228e-1 * t23323 * t1923 - 0.54831135561607547884e-2 * t23327 * t23333 - 0.54831135561607547884e-2 * t23327 * t23337 - 6.0 * t1052 * t23341 + 4.0 * t3169 * t6776 + 0.43864908449286038306e-1 * t23346 * t6707 + t23381 + t23574 + t23732;
    (t23734,)
}
