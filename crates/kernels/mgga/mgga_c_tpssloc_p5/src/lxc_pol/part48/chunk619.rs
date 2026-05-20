//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 619/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk619<F: Float>(t2132: F, t6729: F, t1184: F, t460: F, t2147: F, t478: F, t2131: F, t6739: F, t2133: F, t461: F, t1009: F, t1209: F) -> (F, F, F, F, F, F, F, F) {
    let t7316 = t2132 * t6729;
    let t7319 = t1184 * t460;
    let t7320 = t2147 * t478;
    let t7321 = t7319 * t7320;
    let t7324 = t2131 * t6739;
    let t7325 = t2133 * t461;
    let t7326 = t7324 * t7325;
    let t7327 = t1009 * t1209;
    (t7316, t7319, t7320, t7321, t7324, t7325, t7326, t7327)
}
