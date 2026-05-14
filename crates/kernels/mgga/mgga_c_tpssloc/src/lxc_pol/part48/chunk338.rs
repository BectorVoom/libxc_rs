//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 338/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk338<F: Float>(t2169: F, t3: F, t2028: F, t577: F, t11: F, t2: F, t584: F, t16: F, t9: F, t587: F, t591: F, t14: F, t21: F, t594: F, t598: F, t15: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2170 = t3 * t2169;
    let t2174 = 0.45e1 * t2169 * t577 + t2028;
    let t2218 = 0.174e1 * t11;
    let t2219 = t2 * t584;
    let t2220 = 0.696e1 * t2219;
    let t2221 = t9 * t16;
    let t2222 = 0.1122e2 * t2221;
    let t2223 = t587 * t591;
    let t2224 = 16.0 * t2223;
    let t2225 = t14 * t21;
    let t2226 = 0.778e2 * t2225;
    let t2228 = 0.16272e3 * t594 * t598;
    let t2229 = t15 * t15;
    (t2170, t2174, t2218, t2219, t2220, t2221, t2222, t2223, t2224, t2225, t2226, t2228, t2229)
}
