//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 883/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk883<F: Float>(t1052: F, t1066: F, t3021: F, t3023: F, t3026: F, t3167: F, t3169: F, t3176: F, t3207: F, t388: F, t1068: F, t390: F) -> (F, F, F) {
    let t3209 = F::new(2.0) * t1052 * t3176 - t1052 * t3207 - F::new(2.0) * t1066 * t3026 - F::new(2.0) * t1066 * t3169 + t3021 * t388 + F::new(2.0) * t3023 * t388 + t3167 * t388;
    let t3213 = t1068 * t1068;
    let t3215 = t390 * t390;
    (t3209, t3213, t3215)
}
