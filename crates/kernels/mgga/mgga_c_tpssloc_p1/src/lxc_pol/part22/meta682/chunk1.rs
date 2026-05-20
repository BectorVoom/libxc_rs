//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2248/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2248<F: Float>(t17152: F, t42972: F, t973: F, t10876: F, t13969: F, t17983: F, t13995: F, t14501: F, t10422: F, t18020: F, t3070: F, t10883: F, t17979: F) -> (F, F, F, F, F) {
    let t62766 = t973 * t42972 * t17152;
    let t62778 = t10876 * t13969 * t17983;
    let t62780 = t13995 * t14501;
    let t62811 = t3070 * t10422 * t18020;
    let t62816 = t10883 * t13969 * t17979;
    (t62766, t62778, t62780, t62811, t62816)
}
