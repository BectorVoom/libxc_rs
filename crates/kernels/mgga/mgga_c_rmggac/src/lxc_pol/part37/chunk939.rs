//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 939/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk939<F: Float>(t3281: F, t570: F, t352: F, t76244: F, t77950: F, t77955: F, t77957: F, t77963: F, t77966: F, t77969: F, t77973: F, t77976: F, t77979: F, t77982: F, t77983: F, t8940: F) -> (F, F) {
    let t80444 = t3281 * t570;
    let t80449 = -t77950 + t77955 + t77957 - t77963 + t77966 + t77969 + t77973 + t77976 + 0.11974241701863808564e0 * t8940 * t80444 * t352 + t77979 - t77982 + t77983 - 0.93188427318671584242e-2 * t76244;
    (t80444, t80449)
}
