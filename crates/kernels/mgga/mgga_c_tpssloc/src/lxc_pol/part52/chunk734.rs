//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 734/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk734<F: Float>(t3941: F, t7015: F, t1401: F, t6534: F, t577: F, t671: F, t7002: F, t7010: F, t7014: F, t2108: F, t33: F) -> (F, F, F, F) {
    let t7017 = 27.0 * t3941 * t7015;
    let t7019 = 0.135e2 * t1401 * t6534;
    let t7020 = 0.45e1 * t7002 * t577 + 0.135e2 * t7010 * t671 + t7014 + t7017 + t7019;
    let t7245 = t33 * t2108;
    (t7017, t7019, t7020, t7245)
}
