//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 271/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk271<F: Float>(t290: F, t702: F, t128: F, t618: F, t118: F, t2024: F, t570: F, t551: F, t645: F, t558: F, t2060: F, t1475: F, t236: F, t194: F, t597: F, t201: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2265 = t290 * t702;
    let t2281 = t128 * t618;
    let t2282 = t118 * t2281;
    let t2292 = t2024 * t570;
    let t2295 = t645 * t551;
    let t2298 = t645 * t558;
    let t2301 = t2060 * t570;
    let t2304 = t236 * t1475;
    let t2313 = t194 * t597;
    let t2314 = t2313 * t201;
    (t2265, t2281, t2282, t2292, t2295, t2298, t2301, t2304, t2313, t2314)
}
