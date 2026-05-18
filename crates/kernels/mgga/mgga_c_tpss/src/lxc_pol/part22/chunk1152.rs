//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1152/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1152<F: Float>(t3332: F, t4415: F, t4416: F, t10117: F, t4473: F, t10121: F, t10122: F, t3256: F, t339: F, t790: F, t4419: F, t10086: F, t236: F) -> (F, F, F, F, F) {
    let t12877 = t4415 * t4416 * t3332;
    let t12881 = F::new(7.0) / F::new(576.0) * t10117 * t4473;
    let t12883 = t10121 * t4416 * t10122;
    let t12887 = t339 * t3256 * t790;
    let t12889 = F::new(7.0) / F::new(1152.0) * t12887 * t4419;
    let t12891 = t339 * t10086 * t236;
    (t12877, t12881, t12883, t12889, t12891)
}
