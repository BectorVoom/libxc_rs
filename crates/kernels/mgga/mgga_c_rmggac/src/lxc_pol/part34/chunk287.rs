//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 287/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk287<F: Float>(t2231: F, t82: F, t302: F, t702: F, t290: F, t128: F, t618: F, t118: F, t2024: F, t570: F, t551: F, t645: F) -> (F, F, F, F, F, F, F) {
    let t2232 = t82 * t2231;
    let t2244 = t302 * t702;
    let t2265 = t290 * t702;
    let t2281 = t128 * t618;
    let t2282 = t118 * t2281;
    let t2292 = t2024 * t570;
    let t2295 = t645 * t551;
    (t2232, t2244, t2265, t2281, t2282, t2292, t2295)
}
