//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1232/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1232<F: Float>(t23138: F, t6604: F, t22690: F, t2627: F, t10024: F, t1899: F, t213: F, t6589: F, t9223: F, t22715: F, t229: F, t1891: F, t192: F, t80881: F, t841: F) -> (F, F, F, F, F, F) {
    let t81911 = t23138 * t6604;
    let t81914 = t22690 * t2627;
    let t81920 = t1899 * t10024;
    let t81921 = F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t81920;
    let t81933 = t9223 * t6589 * t213;
    let t81942 = t22715 * t229;
    let t81954 = t80881 * t1891 * t192 * t22690 * t841;
    (t81911, t81914, t81921, t81933, t81942, t81954)
}
