//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 657/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk657<F: Float>(t124: F, t338: F, t22: F, t235: F, t504: F, t7191: F, t14267: F, t71: F, t2227: F, t4616: F, t5542: F, t8601: F) -> (F, F, F, F, F, F) {
    let t36632 = t124 * t338;
    let t36634 = t235 * t36632 * t22;
    let t36639 = t504 * t7191;
    let t36938 = t14267 * t71;
    let t37423 = t4616 * t2227;
    let t38350 = t8601 * t5542;
    (t36632, t36634, t36639, t36938, t37423, t38350)
}
