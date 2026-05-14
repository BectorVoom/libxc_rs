//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1057/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1057<F: Float>(t4918: F, t895: F, t11351: F, t14447: F, t14449: F, t14451: F, t1449: F, t14573: F, t14575: F, t14578: F, t14681: F, t14719: F, t305: F, t3860: F, t3883: F, t4924: F, t8906: F, t905: F) -> (F,) {
    let t14722 = t4918 * t895;
    let t14731 = -0.19751673498613801407e-1 * t14681 - 0.310907e-1 * t14719 * t305 - t14447 + t14449 - t14451 - t14573 - t14575 - t14578 + 0.5848223622634646207e0 * t14722 * t905 + 0.11696447245269292414e1 * t11351 * t1449 + 0.11696447245269292414e1 * t3860 * t3883 - 0.11696447245269292414e1 * t8906 * t4924;
    (t14731,)
}
