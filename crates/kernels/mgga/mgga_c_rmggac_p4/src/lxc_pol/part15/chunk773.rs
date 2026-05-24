//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 773/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk773<F: Float>(t1347: F, t2153: F, t2185: F, t7407: F, t507: F, t8629: F, t124: F, t338: F, t22: F, t235: F, t504: F, t7191: F) -> (F, F, F, F, F) {
    let t36601 = t1347 * t2153;
    let t36612 = t7407 * t2185;
    let t36629 = t507 * t8629;
    let t36632 = t124 * t338;
    let t36634 = t235 * t36632 * t22;
    let t36639 = t504 * t7191;
    (t36601, t36612, t36629, t36634, t36639)
}
