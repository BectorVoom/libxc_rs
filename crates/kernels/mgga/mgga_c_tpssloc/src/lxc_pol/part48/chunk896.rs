//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 896/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk896<F: Float>(t193: F, t7125: F, t2053: F, t40889: F, t10109: F, t7106: F, t2091: F, t40590: F, t12020: F, t7213: F, t2098: F, t2319: F) -> (F, F, F, F, F, F) {
    let t92271 = t193 * t7125;
    let t92394 = t40889 * t2053;
    let t92981 = t10109 * t7106;
    let t93319 = t40590 * t2091;
    let t93818 = t12020 * t7213;
    let t94165 = t2098 * t2319;
    (t92271, t92394, t92981, t93319, t93818, t94165)
}
